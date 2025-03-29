from setuptools import setup, find_packages

setup(
    name='librint',  # Name of the package
    version='0.1.0',  # Version of the package
    packages=find_packages(),  # Automatically find all packages in the pymodule directory
    install_requires=[  # List of dependencies (if any)
        # 'numpy',  # Example: add external dependencies here
    ],
    include_package_data=True,  # Include non-Python files (e.g., static files, templates, etc.)
    classifiers=[
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
    ],
    python_requires='>=3.6',  # Minimum Python version requirement
)
