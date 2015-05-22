def sumMultiple(start, finish, *multiples)
    return (start..finish - 1).select{|i| 
        multiples.any? {|multiple| 
            i % multiple == 0 }
        }.reduce(:+)
end

def next_fib(a, b)
    return a+b
end

def fib(max)
    i = 1
    j = 1
    arr = []
    while i < max do
        arr << next_fib(i, j)
        j = i
        i = i + j
    end
    return arr
end






fib(100).each do |num|
    puts num
end 
# puts sumMultiple(1,1000,3,5)