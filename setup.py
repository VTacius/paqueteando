from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='paqueteando',
    version='0.0.6',
    description='Backup de indices en Elasticsearch 6.x',
    keywords='backup python rust',
    author='Alexander Ortíz',
    author_email='vtacius@gmail.com',
    license='GPLv3',

    packages=["paqueteando"],
    rust_extensions=[RustExtension("paqueteando.paqueteando", binding=Binding.PyO3)],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,

    install_requires=[""],

    test_suite="pytest",

    entry_points = {
        'console_scripts': ['paqueteando=paqueteando.cli:main']
        },

)
