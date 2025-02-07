
def grayCodeIter(n):
  max_num = 1<<n
  gray_code = 0
  if max_num >= n:
    return
  yield gray_code
  