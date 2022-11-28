import os
import toml

def build_cpp_binding(PROJECT_NAME, CXX_LIB_NAME):
	with open("cbindgen.toml", "r") as f:
		cbindgen_conf = toml.load(f)

	include_gaurd = cbindgen_conf["include_guard"]
	maximum_data_len_const = cbindgen_conf["export"]["rename"]["MAXIMUM_DATA_LEN"]
	useless_lines = [f"#ifndef {include_gaurd}", f"#define {include_gaurd}", f"#endif /* {include_gaurd} */", "#ifdef __cplusplus", "#endif // __cplusplus"]

	with open(f"includes/{CXX_LIB_NAME}.h", "r") as f:
		c_lines = f.readlines()

	c_lines = [ line for line in c_lines if line.strip() not in useless_lines ]

	replacement_map = {
		"[[ includes ]]": [ line.strip() for line in c_lines if line.startswith("#include") ],
		"[[ maximum_data_len ]]": [ line.strip().split()[-1] for line in c_lines if line.startswith(f"#define {maximum_data_len_const}") ],
		"[[ c_api ]]": [ line.strip() for line in c_lines if not line.startswith("#") and line.strip() != "" ],
	}

	with open(f"includes/{CXX_LIB_NAME}-template.hpp", "r") as f:
		cpp_lines = f.readlines()

	def cpp_lines_gen():
		for line in cpp_lines:
			match = [ key for key in replacement_map if key in line ]
			if match:
				key = match[0]
				for value in replacement_map[key]:
					yield line.replace(key, value)
			else:
				yield line

	with open(f"includes/{CXX_LIB_NAME}.hpp", "w") as f:
		f.writelines(cpp_lines_gen())
