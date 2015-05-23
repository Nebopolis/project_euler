def palendromic_set(digits)
    max = (10 ** (digits))
    (max * max).downto(0).select {|n| n.to_s.reverse.to_i == n}.to_a
end
    
def palendromic_numbers1(digits)
    max = 10 ** digits - 1
    min = 10 ** (digits - 1)
    results = []
    (min..max).each do |i|
        (min..max).each do |j|
            results << i * j
        end
    end
    (results & palendromic_set(digits)).sort
end

puts palendromic_numbers1(3).last

def palendromic_numbers2(digits)
    max = (10 ** digits) - 1
    min = 10 ** (digits - 1)
    tries = 0
    result = 0
    fac1 = 0
    fac2 = 0
    (min..max).each do |i1|
        (min..max).each do |i2|
            sum = i1 * i2
            if sum == sum.to_s.reverse.to_i && sum > result
                result = sum
                fac1 = i1
                fac2 = i2
            else
                tries = tries + 1
            end
        end 
    end
    puts result
end

palendromic_numbers2(3)

