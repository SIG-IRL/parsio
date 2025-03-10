from setuptools import setup
import os

setup(
    name="parsio",
    version="0.1.0",
    description="A powerful Persian NLP toolkit",
    long_description=open("README.md").read() if os.path.exists("README.md") else "",
    long_description_content_type="text/markdown",
    license="MIT",  
    python_requires=">=3.8",
    packages=["parsio"],
    package_dir={"": "src"},
    install_requires=[],
    zip_safe=False,
)
