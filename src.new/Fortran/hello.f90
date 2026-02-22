program hello
  implicit none
  integer :: l, m, ans, x, y, i
  integer, dimension(10005) :: len
  read(*, *) l, m
  len(:) = 0
  ans = 0
  do i = 0, m - 1
    read(*, *) x, y
    len(x) = len(x) + 1
    len(y + 1) = len(y + 1) - 1
  end do
  do i = 1, l + 1
    if (i /= 0) then
      len(i) = len(i - 1) + len(i)
    end if
    if (len(i) == 0) then
      ans = ans + 1
    end if
  end do
  print *, ans
end program hello