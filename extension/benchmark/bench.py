from benchmarker import Benchmarker
import fast_woothee_pyo3
import fast_woothee_cpy

pyo3_parse = fast_woothee_pyo3.parse
cpy_parse = fast_woothee_cpy.parse
ua = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.104 Safari/537.36'


n = 1000 * 1000

with Benchmarker(n) as bench:

    @bench("rust-cpython")
    def _cpython(bm):
        parse = cpy_parse
        for i in bm:
            parse(ua)

    @bench("pyo3")
    def _pyo3(bm):
        parse = pyo3_parse
        for i in bm:
            parse(ua)
