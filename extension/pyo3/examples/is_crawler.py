import fast_woothee_pyo3 as woothee

ua = "Mozilla"
ret = woothee.is_crawler(ua)
print(ret)

ua = "Mozilla/4.0 (compatible; MSIE 7.0; Windows Phone OS 7.5; Trident/3.1; IEMobile/7.0; FujitsuToshibaMobileCommun; IS12T; KDDI)"
ret = woothee.parse(ua)
print(ret)
