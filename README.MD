# AguMatrix

## A simple, cross-platform program for solving determinants
AguMatrix uses the Laplace expansion method to calculate the determinant of square matrices up to a size of 256x256.

### Issues
The parser cannot distinguish rows or columns with the improper number of elements, nor it can detect if the matrix is not square. The size of the matrix is inferred from the length of its first row. When the number of elements is mismatched, the empty spaces are filled with 0s.

### User Interface
The GUI for AguMatrix is made with ICED, a cross-platform (Windows, Mac OS, Linux and Web) GUI library for Rust.
