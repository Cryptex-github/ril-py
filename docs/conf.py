# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
import re

# -- Project information -----------------------------------------------------

project = 'ril'
copyright = '2022, Cryptex' 
author = 'Cryptex'

# The full version, including alpha/beta/rc tags

# with open('../pyproject.toml') as f:
#     matches = re.search(r'^version\s*=\s*[\'"]([^\'"]*)[\'"]', f.read(), re.MULTILINE)

#     if matches:
#         release = matches.group(0)
#     else:
#         raise RuntimeError('Unable to find version string in pyproject.toml')

release = '0.4.0'


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.extlinks',
    'sphinx.ext.intersphinx',
    'sphinx.ext.napoleon',
    'sphinxext.opengraph',
    'sphinx_copybutton',
]

autodoc_typehints = 'both'
napoleon_google_docstring = False
napoleon_numpy_docstring = True
autodoc_member_order = 'bysource'

# Add any paths that contain templates here, relative to this directory.
templates_path = []

rst_prolog = """
.. |enum| replace:: This is an |enum_link|_.
.. |enum_link| replace:: *enum*
.. _enum_link: https://docs.python.org/3/library/enum.html#enum.Enum
"""

intersphinx_mapping = {
    'py': ('https://docs.python.org/3', None),
}

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = []

pygments_style = "friendly"


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'furo'

html_theme_options = {}



# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
# html_static_path = ["./_static"]
