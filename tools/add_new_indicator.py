from typing import List
from pathlib import Path
import argparse
from utils.logger import logger
from string import Template

ROOT = Path(__file__).parent.parent

# TEMPLATES
TEMPLATE_DIR = ROOT / "tools" / "templates"
INSERT_TO_PYLIB = ROOT / "tools" / "templates" / "insert_pylib.template"
INSERT_TO_FUZZ_CARGO = ROOT / "tools" / "templates" / "insert_fuzz_cargo.template"

# WILL BE MODIFIED
#   BENCHMARKS
BENCH_PYTHON_TIMEIT = ROOT / "benchmarks/python/benchmark_timeit.py"
#   CRATES
CORE_LIB_PATH =  ROOT / "crates/core/src/indicators/mod.rs"
PYBINDING_LIB_PATH = ROOT / "crates/python/src/lib.rs"
#   PYTHON
PYTHON_STUB_INIT = ROOT / "python/technicalysis/_core/__init__.pyi"
MAPTYPES_PYTHON = ROOT / "python/technicalysis/maptypes.py"
#   TESTS
FUZZ_TESTS_CARGO = ROOT / "tests/fuzz/Cargo.toml"
RUST_TEST_LIB_PATH = ROOT / "tests/rust/src/lib.rs"

# WILL BE CREATED
TEMPLATES_TO_FILE = {
    "bench_python_pyperf.template": "benchmarks/python/pyperf_bench/bench_{name}.py",
    "bench_python_timeit.template": "benchmarks/python/timeit_bench/{name}.py",
    "bench_rust.template": "benchmarks/rust/indicators/bench_{name}.rs",
    "crate_core.template": "crates/core/src/indicators/{name}.rs",
    "crate_python.template": "crates/python/src/py_{name}.rs",
    "python_stub.template": "python/technicalysis/_core/{name}.pyi",
    "tests_fuzz.template": "tests/fuzz/fuzz_targets/fuzz_{name}.rs",
    "tests_python.template": "tests/python/test_{name}.py",
    "tests_rust.template": "tests/rust/src/tests_{name}.rs",
}

def create_from_template(indicator_name: str, indicator_camel_case: str = None):
    for template_file, target_file in TEMPLATES_TO_FILE.items():
        template_path = TEMPLATE_DIR / template_file
        target_path = Path(__file__).parent.parent / target_file.format(name=indicator_name)
        target_path.parent.mkdir(parents=True, exist_ok=True)
        with open(template_path, 'r') as template_f:
            template_content = Template(template_f.read())
        with open(target_path, 'w') as target_f:
            target_f.write(template_content.substitute({
                "indicator_name": indicator_name,
                "IndicatorName": indicator_name.capitalize() if (indicator_camel_case is None) else indicator_camel_case,
            }))
        logger.info(f"➕ Created {target_path}")

def add_to_bench_timeit(indicator_name: str):
    with open(BENCH_PYTHON_TIMEIT, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("from timeit_bench")), len(lines))
    lines.insert(insert_position, f"from timeit_bench.{indicator_name} import benchmark_{indicator_name}\n")

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("BENCHMARKS = {")), len(lines))
    lines.insert(insert_position+1, f"    '{indicator_name}': benchmark_{indicator_name},\n")

    with open(BENCH_PYTHON_TIMEIT, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {BENCH_PYTHON_TIMEIT}")

def add_to_core(indicator_name: str):
    with open(CORE_LIB_PATH, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("pub mod ")), len(lines))

    lines.insert(insert_position, f"pub mod {indicator_name};\n")

    with open(CORE_LIB_PATH, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {CORE_LIB_PATH}")

def add_to_pybindings(indicator_name: str, indicator_camel_case: str = None):
    with open(PYBINDING_LIB_PATH, 'r') as f:
        lines = f.readlines()
    
    wrap_insertion = next((i for i, line in enumerate(lines) if line.startswith("Ok(())")), len(lines))
    with open(INSERT_TO_PYLIB, 'r') as template_f:
        template_content = Template(template_f.read())
    lines.insert(wrap_insertion-2, template_content.substitute({
        "indicator_name": indicator_name,
        "IndicatorName": indicator_name.capitalize() if (indicator_camel_case is None) else indicator_camel_case,
    }))
    
    mod_insertion = next((i for i, line in enumerate(lines) if line.startswith("mod ")), len(lines))
    lines.insert(mod_insertion, f"mod py_{indicator_name};\n")

    with open(PYBINDING_LIB_PATH, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {PYBINDING_LIB_PATH}")

def add_to_python_stub_init(indicator_name: str):
    with open(PYTHON_STUB_INIT, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("from .")), len(lines))
    lines.insert(insert_position, f"from .{indicator_name} import *\n")

    with open(PYTHON_STUB_INIT, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {PYTHON_STUB_INIT}")

def add_to_fuzz_tests_cargo(indicator_name: str):
    with open(FUZZ_TESTS_CARGO, 'r') as f:
        lines = f.readlines()

    with open(INSERT_TO_FUZZ_CARGO, 'r') as template_f:
        template_content = Template(template_f.read())
    lines.insert(len(lines), template_content.substitute({
        "indicator_name": indicator_name,
    }))

    with open(FUZZ_TESTS_CARGO, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {FUZZ_TESTS_CARGO}")

def add_rust_test(indicator_name: str):
    with open(RUST_TEST_LIB_PATH, 'r') as f:
        lines = f.readlines()

    lines.insert(len(lines), f"#[cfg(test)]\npub(crate) mod tests_{indicator_name};\n")

    with open(RUST_TEST_LIB_PATH, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {RUST_TEST_LIB_PATH}")

def add_maptypes(indicator_name: str):
    with open(MAPTYPES_PYTHON, 'r') as f:
        lines = f.readlines()

    insert_position = next((i for i, line in enumerate(lines) if line.startswith("FCT_TO_NAMEDTUPLE = {")), len(lines))
    lines.insert(insert_position + 1, f'    "{indicator_name}": namedtuple("{indicator_name.capitalize()}Result", [... ,"state"]), #TODO: ADD OUTPUTS ARGS\n')

    with open(MAPTYPES_PYTHON, 'w') as f:
        f.writelines(lines)
    
    logger.info(f"🔄 Updated {MAPTYPES_PYTHON}")

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("name", type=str)
    parser.add_argument(
        "-c",
        "--camel_case",
        type=str
    )
    return parser.parse_args()

def main():
    args = parse_args()
    logger.info(f"✨ Adding new indicator template: {args.name}")
    create_from_template(args.name, args.camel_case)
    add_to_bench_timeit(args.name)
    add_to_python_stub_init(args.name)
    add_to_fuzz_tests_cargo(args.name)
    add_rust_test(args.name)
    add_to_core(args.name)
    add_to_pybindings(args.name, args.camel_case)
    add_maptypes(args.name)
    logger.info(f"✅ Successfully added new indicator template: {args.name}")

if __name__ == "__main__":
    main()