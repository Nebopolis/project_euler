require 'prime'

def prime_factors(n) 
    Prime.prime_division(n).flat_map { |factor, power| factor }
end

def prime_factors2(number)
    for factor in 2..number
        number /= factor while number > factor && (number % factor).zero?
        break if number == factor
    end
    number
end

puts prime_factors2(13195)
puts prime_factors2(600851475143)

puts prime_factors(13195).last
puts prime_factors(600851475143).last
