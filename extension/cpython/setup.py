from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(name='fast-woothee-cpy',
      version='0.1',
      rust_extensions=[
          RustExtension('fast_woothee_cpy', 'Cargo.toml',
                        binding=Binding.RustCPython)],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
