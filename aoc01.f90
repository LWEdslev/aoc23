function first_spelled_digit_index(s) result(out)
  implicit none
  integer :: out, i, j, length
  character(100) :: substring
  character(*), intent(in) :: s
  character(10), dimension(0:9) :: numbers
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
  out =10000
  
  outer: do i = 0, len(s), 1
    do j = 0, 9, 1
      length = len(trim(numbers(j)))
      substring = s(i:(i+length-1))
      if (trim(substring) == trim(numbers(j))) then
        out = i
        exit outer
      end if
    end do
  end do outer
end function

function first_spelled_digit(s) result(out)
  implicit none
  integer :: out, i, j, length
  character(100) :: substring
  character(*), intent(in) :: s
  character(10), dimension(0:9) :: numbers
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
  out = 10000
  
  outer: do i = 0, len(s), 1
    do j = 0, 9, 1
      length = len(trim(numbers(j)))
      substring = s(i:(i+length-1))
      if (trim(substring) == trim(numbers(j))) then
        out = j
        exit outer
      end if
    end do
  end do outer
end function

function last_spelled_digit(s) result(out) 
  implicit none
  integer :: out, i, j, length
  character(100) :: substring
  character(*), intent(in) :: s
  character(10), dimension(0:9) :: numbers
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
  out = -1
  
  outer: do i = len(s), 0, -1
    do j = 0, 9, 1
      length = len(trim(numbers(j)))
      substring = s(i:(i+length-1))
      if (trim(substring) == trim(numbers(j))) then
        out = j
        exit outer
      end if
    end do
  end do outer
end function


function last_spelled_digit_index(s) result(out) 
  implicit none
  integer :: out, i, j, length
  character(100) :: substring
  character(*), intent(in) :: s
  character(10), dimension(0:9) :: numbers
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
  out = -1
  
  outer: do i = len(s), 0, -1
    do j = 0, 9, 1
      length = len(trim(numbers(j)))
      substring = s(i:(i+length-1))
      if (trim(substring) == trim(numbers(j))) then
        out = i 
        exit outer
      end if
    end do
  end do outer
end function

function find_first_digit(s) result(out)
  implicit none
  integer :: out
  integer :: i, j
  character(*), intent(in) :: s
  character(10) :: digitss
  out = 10000
  digitss = '0123456789'
  outer: do i = 1, len(s)-1
    do j = 0, 10, 1
      if (s(i:i) == digitss(j:j)) then
        out = j-1
        exit outer
      end if
    end do
  end do outer
end function find_first_digit

function find_first_digit_index(s) result(out)
  implicit none
  integer :: out
  integer :: i, j
  character(*), intent(in) :: s
  character(10) :: digitss
  out = 10000
  digitss = '0123456789'
  outer: do i = 1, len(s)-1
    do j = 0, 10, 1
      if (s(i:i) == digitss(j:j)) then
        out = i
        exit outer
      end if
    end do
  end do outer
end function find_first_digit_index

function find_last_digit(s) result(out)
  implicit none
  integer :: out, i, j
  character(*), intent(in) :: s
  character(100) :: digitss
  out = -1

  digitss = '0123456789'
  outer: do i = len(s), 0, -1
    do j = 0, 10, 1
      if (s(i:i) == digitss(j:j)) then
        out = j-1
        exit outer
      end if
    end do
  end do outer
end function find_last_digit  

function find_last_digit_index(s) result(out)
  implicit none
  integer :: out, i, j
  character(*), intent(in) :: s
  character(100) :: digitss
  out = -1

  digitss = '0123456789'
  outer: do i = len(s), 0, -1
    do j = 0, 10, 1
      if (s(i:i) == digitss(j:j)) then
        out = i
        exit outer
      end if
    end do
  end do outer
end function find_last_digit_index 

program aoc01
    implicit none
    integer :: find_first_digit, find_first_digit_index, find_last_digit, find_last_digit_index
    integer:: last_spelled_digit, last_spelled_digit_index, first_spelled_digit, first_spelled_digit_index
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
        write(*,*) 'Read line: ', trim(line)
        first_d = find_first_digit(line)
        first_d_i = find_first_digit_index(line)
        last_d = find_last_digit(line)
        last_d_i = find_last_digit_index(line)
        first_w = first_spelled_digit(line)
        first_w_i = first_spelled_digit_index(line)
        last_w = last_spelled_digit(line)
        last_w_i = last_spelled_digit_index(line)

        
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
      write(*,*) sum
      close(unit_number, status='keep')
end program
