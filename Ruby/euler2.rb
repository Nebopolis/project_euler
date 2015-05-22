def sumMultiple(seq, *multiples)
    seq.select{|i|
        multiples.any? {|multiple|
            i % multiple == 0 }
        }.reduce(:+)
end

fib = Enumerator.new do |yielder|
    i1, i2 = 1, 1
    loop do
        yielder.yield i1
        i1, i2 = i2, i1 + i2
    end
end

puts sumMultiple(fib.take_while {|n| n < 4000000}, 2)

#
# Learning how ruby deals with iterators and blocks
#
# def fib2(n)
#     i1, i2 = 1, 1
#     while i1 <= n
#         yield i1
#         i1, i2 = i2, i1 + i2
#     end
# end

# def fib3()
#     i1, i2 = 1, 1
#     loop do
#         yield i1
#         i1, i2 = i2, i1 + i2
#     end
# end

# fib2(5) {|n| puts n}

# fib3() do |n|
#     if n > 5
#         break
#     else
#         puts n
#     end
# end
