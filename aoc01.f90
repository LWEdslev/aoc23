subroutine spelled_digit(s, out, outi, first)
  implicit none
  integer, intent(out) :: out, outi
  integer :: i, j, length
  character(100) :: substring
  character(*), intent(in) :: s
  logical, intent(in) :: first
  character(10), dimension(0:9) :: numbers
  integer :: starti, endi, inc
  numbers(0) = 'zero'
  numbers(1) = 'one'
  numbers(2) = 'two'
  numbers(3) = 'three'
  numbers(4) = 'four'
  numbers(5) = 'five'
  numbers(6) = 'six'
  numbers(7) = 'seven'
  numbers(8) = 'eight'
  numbers(9) = 'nine'
  outi = -1
  starti = len(s)
  endi = 0
  inc = -1

  if (first) then
    outi =10000
    starti = 0
    endi = len(s)
    inc = 1
  end if

  outer: do i = starti, endi, inc
    do j = 0, 9, 1
      length = len(trim(numbers(j)))
      substring = s(i:(i+length-1))
      if (trim(substring) == trim(numbers(j))) then
        out = j
        outi = i
        exit outer
      end if
    end do
  end do outer
end subroutine

subroutine find_digit(s, out, outi, first) 
  implicit none
  integer, intent(out) :: out, outi
  integer :: i, j
  character(*), intent(in) :: s
  logical, intent(in) :: first
  integer :: starti, endi, inc
  character(10) :: digitss
  outi = 10000
  digitss = '0123456789'

  starti = len(s)-1
  endi = 1
  inc = -1
  if (first) then
    starti = 1
    endi = len(s)-1
    inc = 1
  end if
  outer: do i = starti, endi, inc
    do j = 0, 10, 1
      if (s(i:i) == digitss(j:j)) then
        out = j-1
        outi = i
        exit outer
      end if
    end do
  end do outer
end subroutine

program aoc01
    implicit none
    
    character(100) :: line
    integer :: unit_number, status, res, sum
    integer :: first_d, first_d_i, last_d, last_d_i, first_w, first_w_i, last_w, last_w_i
    integer :: first, last
    open(unit=unit_number, file='aoc01.txt', action='read', iostat=status)
    sum = 0
    do
        read(unit_number, '(A)', iostat=status) line
    
        if (status /= 0) then
          if (status == -1) then
            exit
          else
            write(*,*) 'Error reading file'
            stop
          end if
        end if

        call find_digit(line, first_d, first_d_i, .true.)
        call find_digit(line, last_d, last_d_i, .false.)

        call spelled_digit(line, first_w, first_w_i, .true.)
        call spelled_digit(line, last_w, last_w_i, .false.)

        if (first_d_i < first_w_i) then
          first = first_d
        else 
          first = first_w
        end if

        if (last_d_i > last_w_i) then
          last = last_d
        else 
          last = last_w
        end if

        res = (first * 10) + last
        sum = sum + res
      end do
      write(*,'(I0)') sum
      close(unit_number, status='keep')
end program
