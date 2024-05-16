Subject: https://cdn.intra.42.fr/pdf/pdf/113262/en.subject.pdf \
Write a series of function to do vectors and matrices operations

References used:
- Column major order: https://en.wikipedia.org/wiki/Row-_and_column-major_order
- Infinite/Supremum norm: https://en.wikipedia.org/wiki/Uniform_norm
- Abs of complex number: https://www.varsitytutors.com/hotmath/hotmath_help/topics/absolute-value-complex-number
- Approximation of square root using the babylonian method: https://blogs.sas.com/content/iml/2016/05/16/babylonian-square-roots.html
- Cosine of 2 vectors: https://www.geeksforgeeks.org/cosine-similarity/
- Multiplication of matrices/vectors: https://mathinsight.org/matrix_vector_multiplication
- Row-echelon form: https://www.statlect.com/matrix-algebra/row-echelon-form / https://www.wikihow.com/Reduce-a-Matrix-to-Row-Echelon-Form / https://www.emathhelp.net/en/calculators/linear-algebra/reduced-row-echelon-form-rref-calculator/

Rust version: `rustc 1.77.2`

Libraries used:

Completion:
- [x] ex00 add, sub, scl
- [x] ex01 linear combination
- [x] ex02 lerp
- [x] ex03 dot
- [x] ex04 norm
- [x] ex05 cos
- [x] ex06 cross
- [x] ex07 linear map, mat mult
- [x] ex08 trace
- [x] ex09 transpose
- [x] ex10 row echelon form
- [x] ex11 determinant
- [ ] ex12 inverse
- [ ] ex13 rank
- [ ] ex14 projection
- [ ] ex15 complex vector spaces

Testing:
`cargo test`