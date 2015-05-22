def sumMultiple(start, finish, *multiples)
    return (start..finish - 1).select{|i| 
        multiples.any? {|multiple| 
            i % multiple == 0 }
        }.reduce(:+)
end



puts sumMultiple(1,10,3,5)
puts sumMultiple(1,1000,3,5)