from setuptools import setup, find_packages

setup(
    name='librint',
    version='0.1.0',
    packages=find_packages(),
    install_requires=[
        "basis-set-exchange>=0.11",
        "matplotlib>=3.10.8",
        "numpy>=2.2.6",
        "pyscf>=2.12.1",
        "pyscfad>=0.1.10",
        "scipy>=1.15.3",
    ],
    include_package_data=True,
    classifiers=[
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
    ],
    python_requires='>=3.6',
)
