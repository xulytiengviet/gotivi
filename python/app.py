"""FastAPI example via the GoTiVi Rust C ABI. Requires cargo build --release."""
import ctypes
import os
from pathlib import Path
from fastapi import FastAPI
from pydantic import BaseModel
root = Path(__file__).resolve().parent.parent
libname = "gotivi.dll" if os.name == "nt" else ("libgotivi.dylib" if os.uname().sysname == "Darwin" else "libgotivi.so")
lib = ctypes.CDLL(str(root / "target" / "release" / libname))
class Action(ctypes.Structure):
    _fields_ = [("kind",ctypes.c_uint32),("delete_utf16",ctypes.c_uint32),("cursor_utf16",ctypes.c_uint32),("pass_key",ctypes.c_bool),("text",ctypes.c_char_p)]
lib.gotivi_new.argtypes = [ctypes.c_uint32]
lib.gotivi_new.restype = ctypes.c_void_p
lib.gotivi_feed.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
lib.gotivi_feed.restype = Action
lib.gotivi_free.argtypes = [ctypes.c_void_p]
app = FastAPI(title="GoTiVi API")
class Request(BaseModel):
    text: str
    mode: str = "telex"
@app.post("/api/type")
def type_text(req:Request):
    mode={"telex":0,"simple":1,"vni":2,"viqr":3,"english":4}.get(req.mode.lower(),0)
    s=lib.gotivi_new(mode)
    output=""
    try:
        for char in req.text:
            if char.isspace() or not char.isascii():
                output+=char
                lib.gotivi_feed(s,b" ")
            else:
                a=lib.gotivi_feed(s,char.encode("utf-8"))
                if a.kind == 1:
                    n=a.delete_utf16
                    while n and output:
                        old=output[-1];output=output[:-1]
                        n -= len(old.encode("utf-16-le"))//2
                    output+=(a.text or b"").decode("utf-8")
                elif a.pass_key: output+=char
        return {"output":output,"mode":req.mode}
    finally:
        lib.gotivi_free(s)
