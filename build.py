import os

base_address = 0x80400000
step = 0x20000
linker = "src/linker.ld"

app_id = 0
apps = os.listdir("build/app")
apps.sort()
chapter = os.getenv("CHAPTER")

for app in apps:
    app = app[: app.find(".")]
    os.system(
        # -Clink-args=-Ttext=%x参数指定了链接器参数，其中-C是编译器选项的前缀，link-args表示编译器应该将这些参数传递给链接器。-Ttext=%x指定了链接器应该将.text段加载到内存中的指定地址，其中%x是一个占位符，它将在运行时被替换为实际的内存地址。
        "cargo rustc --bin %s --release -- -Clink-args=-Ttext=%x"
        % (app, base_address + step * app_id)
    )
    print(
        "[build.py] application %s start with address %s"
        % (app, hex(base_address + step * app_id))
    )
    if chapter == '3':
        app_id = app_id + 1
