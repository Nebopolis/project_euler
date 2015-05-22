def sumMultiple(start, finish, *multiples)
    (start..finish - 1).select do |i|
        multiples.any? do |multiple|
            i % multiple == 0 
        end
    end.reduce(:+)
end

puts sumMultiple(1,10,3,5)
puts sumMultiple(1,1000,3,5)
