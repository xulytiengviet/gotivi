"""GoTiVi reference FastAPI adapter. The language engine is the Rust cdylib."""
import ctypes
import os
import sys
from pathlib import Path

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field

ROOT = Path(__file__).resolve().parent.parent
LIB_NAME = ("gotivi.dll" if os.name == "nt" else
            "libgotivi.dylib" if sys.platform == "darwin" else "libgotivi.so")
LIB = ctypes.CDLL(str(ROOT / "target" / "release" / LIB_NAME))

class Action(ctypes.Structure):
    _fields_ = [
        ("kind", ctypes.c_uint32),
        ("revision", ctypes.c_uint32),
        ("delete_utf16", ctypes.c_uint32),
        ("pass_key", ctypes.c_bool),
        ("text", ctypes.c_char_p),
        ("expected", ctypes.c_char_p),
    ]

LIB.gotivi_new.argtypes = [ctypes.c_uint32]
LIB.gotivi_new.restype = ctypes.c_void_p
LIB.gotivi_free.argtypes = [ctypes.c_void_p]
LIB.gotivi_feed.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
LIB.gotivi_feed.restype = Action
LIB.gotivi_ack.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_bool]
LIB.gotivi_ack.restype = ctypes.c_bool

app = FastAPI(title="GoTiVi Reference API", version="0.2.0")

class TypeRequest(BaseModel):
    text: str = Field(max_length=10_000)
    mode: str = Field(default="telex", pattern="^(telex|simple|vni|viqr|english)$")

@app.post("/api/type")
def type_text(req: TypeRequest):
    mode = {"telex": 0, "simple": 1, "vni": 2, "viqr": 3, "english": 4}[req.mode]
    session = LIB.gotivi_new(mode)
    if not session:
        raise HTTPException(status_code=500, detail="Cannot initialize engine")
    text = ""
    try:
        for key in req.text:
            a = LIB.gotivi_feed(session, key.encode("utf-8"))
            if a.kind in (1, 2, 3):
                expected = (a.expected or b"").decode("utf-8")
                inserted = (a.text or b"").decode("utf-8")
                delete_units = len(expected.encode("utf-16-le")) // 2
                if not text.endswith(expected) or (a.delete_utf16 not in (0, delete_units)):
                    LIB.gotivi_ack(session, a.revision, False)
                    raise HTTPException(status_code=409, detail="Host text mismatch")
                text = text[:len(text)-len(expected)] + inserted if expected else text + inserted
                if not LIB.gotivi_ack(session, a.revision, True):
                    raise HTTPException(status_code=409, detail="Revision mismatch")
            elif a.kind == 4:
                raise HTTPException(status_code=409, detail="Engine awaiting ACK")
            elif a.pass_key:
                text += key
        # Finalize last word so macros apply at the end, without inserting a
        # synthetic delimiter into the output returned to API callers.
        if req.text and req.text[-1].isascii() and req.text[-1].isalpha():
            a = LIB.gotivi_feed(session, b" ")
            if a.kind == 3:
                expected = (a.expected or b"").decode("utf-8")
                committed = (a.text or b"").decode("utf-8")
                if not text.endswith(expected):
                    LIB.gotivi_ack(session, a.revision, False)
                    raise HTTPException(status_code=409, detail="Host text mismatch on commit")
                text = text[:len(text)-len(expected)] + committed[:-1]
                LIB.gotivi_ack(session, a.revision, True)
        return {"output": text, "mode": req.mode, "engine": "rust-c-abi"}
    finally:
        LIB.gotivi_free(session)
