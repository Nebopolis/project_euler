require 'prime'

def prime_factors(n) 
    Prime.prime_division(n).flat_map { |factor, power| factor }
end

def even_factor(range)
    result = 1
    (1..range).each do |number|
        if not (result % number).zero?
            prime_factors(number).each do |factor|
                result = result * factor
            end
        end
    end
    result
end

def test_factor(range)
    result = even_factor(range)
    (1..range).each do |number|
        if not (result % number).zero?
            return false
        end
    end
    true
end

n = 10

puts even_factor(n)
puts test_factor(n, even_factor(n))

n = 20

puts even_factor(n)
puts test_factor(n, even_factor(n))
