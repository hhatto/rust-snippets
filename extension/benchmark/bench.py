from benchmarker import Benchmarker
import fast_woothee_pyo3
import fast_woothee_cpy

ua = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.104 Safari/537.36'


n = 1000 * 1000

with Benchmarker(n) as bench:

    @bench("(rust-cpython) parse")
    def _cpython(bm):
        parse = fast_woothee_cpy.parse
        for i in bm:
            parse(ua)

    @bench("(pyo3) parse")
    def _pyo3(bm):
        parse = fast_woothee_pyo3.parse
        for i in bm:
            parse(ua)

    @bench("(rust-cpython) parse2")
    def _cpython(bm):
        parse = fast_woothee_cpy.parse2
        for i in bm:
            parse(ua)

    @bench("(pyo3) parse2")
    def _pyo3(bm):
        parse = fast_woothee_pyo3.parse2
        for i in bm:
            parse(ua)

    @bench("(rust-cpython) is_crawler")
    def _cpython(bm):
        parse = fast_woothee_cpy.is_crawler
        for i in bm:
            parse(ua)

    @bench("(pyo3) is_crawler")
    def _pyo3(bm):
        parse = fast_woothee_pyo3.is_crawler
        for i in bm:
            parse(ua)
