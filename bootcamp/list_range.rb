# You have k lists of sorted integers. Find the smallest range that includes at least one number from each of the k lists.

# For example, 
# List 1: [4, 10, 15, 24, 26]
# List 2: [0, 9, 12, 20]
# List 3: [5, 18, 22, 30]

# The smallest range here would be [20, 24] as it contains 24 from list 1, 20 from list 2, and 22 from list 3.

def smallest_range(lists)
  numbers = lists.flatten(1).sort
  min_distance = numbers.last - numbers.first
  min_start = numbers.first
  min_stop = numbers.last

  def valid(start, stop, lists)
    lists.all? do |list|
      list.any? do |number|
        number >= start && number <= stop
      end
    end
  end 

  numbers.each do |start|
    numbers.each do |stop|
      if stop - start <= min_distance && stop > start && valid(start, stop, lists)
        min_distance = stop - start
        min_start = start
        min_stop = stop
      end
    end
  end

  [min_start, min_stop]
end

lists = [[4, 10, 15, 24, 26],[0, 9, 12, 20],[5, 18, 22, 30]]
start, stop = smallest_range lists
puts "The best solution is Start: #{start} and Stop: #{stop}"

####
# Supplied solution:
# This can be solved easily as below.
# 1. initialize smallest_range as MAX_INT
# 2. keep 3 pointers/index p1, p2 and p3 which points to the first elements of lists L1, L2 and L3 respectively.
# 3. find the max value and min value pointed/indexed by p1, p2 and p3
# 4. difference of max value and min value discovered in step 3 is the current range. compare it with smallest_range and update it, if found smaller.
# 5. increment the pointer/index of min value found in step 3.
# 6. repeat step 3 to 5 until the pointer/index of min value is in range. 

def alternate_solution(lists)
  @lists = lists
  @smallest_range = 99999999999
  @index_a, @index_b, @index_c = [0,0,0]
  start, stop = [0,0]

  def step()
    max = 0
    min = 0
    a = @lists[0][@index_a]
    b = @lists[1][@index_b]
    c = @lists[2][@index_c]
    if a > b && a > c
      max = a
    elsif b > a && b > c
      max = b
    else 
      max = c
    end
    if a < b && a < c
      min = a
      @index_a += 1
    elsif b < a && b < c
      min = b
      @index_b += 1
    else 
      min = c
      @index_c += 1
    end
    range = max - min
    @smallest_range = range if @smallest_range > range
    [min, max]
  end
  while @index_a < lists[0].length && @index_b < lists[1].length && @index_c < lists[2].length
    start, stop = step()
  end
  [start, stop]
end

start, stop = alternate_solution lists
puts "The best solution is Start: #{start} and Stop: #{stop}"


