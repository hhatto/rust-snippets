from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(name='fast-woothee-pyo3',
      version='0.1',
      rust_extensions=[
          RustExtension('fast_woothee_pyo3', 'Cargo.toml',
                        binding=Binding.PyO3)],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
