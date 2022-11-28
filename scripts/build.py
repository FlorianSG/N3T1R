import os
from build_cpp_binding import build_cpp_binding

PROJECT_NAME = "N3T1R"
CXX_LIB_NAME = PROJECT_NAME.lower()

print(f"=== Building <{PROJECT_NAME}> ===")

os.chdir(os.path.dirname(os.path.dirname(__file__)))

os.system("cargo build --target=x86_64-pc-windows-gnu")

build_cpp_binding(PROJECT_NAME, CXX_LIB_NAME)

os.system(f"copy includes\\{CXX_LIB_NAME}.hpp C:\\msys64\\home\\Florian\\melonDS\\src\\{CXX_LIB_NAME}")
os.system(f"copy target\\x86_64-pc-windows-gnu\\debug\\{CXX_LIB_NAME}.dll C:\\msys64\\home\\Florian\\melonDS\\lib")