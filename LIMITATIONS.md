# GoTiVi 0.2.0 — giới hạn đã biết

- **Chưa phải IME dùng toàn hệ thống.** Không có driver Windows TSF/hook, macOS IMK, Linux Fcitx5/IBus, Android/iOS hay UEFI.
- Phân tích âm tiết hiện là heuristic, **chưa có** từ điển tần suất, quy tắc âm vị học hoàn chỉnh, kiểm tra nguyên âm/phụ âm nâng cao, hay mô hình bảo vệ từ tiếng Anh mạnh.
- Telex, VNI và VIQR còn các góc cạnh chưa bao phủ như phím dấu lặp phức tạp, dấu câu VIQR, kiểu đặt dấu ở một số cụm đặc biệt, phím nhanh và chuyển đổi qua lại Unicode/NFD đầy đủ từ raw nguồn hỗn hợp.
- `MAX_RAW=24` chỉ giới hạn bộ đệm từ; tràn bộ đệm sẽ chốt một đoạn rồi bắt đầu đoạn mới. Chưa có chỉnh sửa từ **đã chốt** theo ngữ cảnh câu.
- `ack(revision, accepted)` phải được adapter gọi đồng bộ hoặc theo hàng đợi. Khi `Busy`, adapter phải retry/buffer phím. Rejected edit phải reset/resync; không tự đoán vị trí con trỏ.
- `MemoryHost` chỉ là mock tham chiếu; vị trí con trỏ và selection thực tế của host cần được xác minh bởi adapter gốc.
- C ABI nhận con trỏ thô: chỉ dùng với mã native tin cậy, trên đúng luồng, không sử dụng pointer của action sau khi gọi hàm FFI tiếp theo.
- Browser playground gắn vào một capture field, không tương đương hệ thống IME.
- API Python chỉ là bản tham chiếu; cần cấu hình bảo mật, CORS và giới hạn request trước khi đưa lên internet.
- Số ca kiểm thử thực tế là số pytest/cargo test trong GitHub Actions; mục tiêu 13.337 + 3.000 **chưa được công nhận là đạt**.

GoTiVi không chứa, đóng gói hoặc phân phối binary ViFastKey được cung cấp để tham khảo.
