# There is an island which is represented by square matrix NxN. 
# A person on the island is standing at any given co-ordinates (x,y). He can move in any direction one step right, left, up, down on the island. If he steps outside the island, he dies.

# Let island is represented as (0,0) to (N-1,N-1) (i.e NxN matrix) & person is standing at given co-ordinates (x,y). He is allowed to move n steps on the island (along the matrix). What is the probability that he is alive after he walks n steps on the island?

# Write a psuedocode &amp; then full code for function
#  " float probabilityofalive(int x,int y, int n) ".
m = "man"

[[0,0,0,0],
 [0,0,0,0],
 [0,0,m,0],
 [0,0,0,0]]

n = 2
x = 0
y = 0
[[m,0],
 [0,0]]

[[m,0.25],
 [0.25,0]]

[[0.125,0],
 [0,0.125]]


[[0,0,0,0],
 [0,0,1,0],
 [0,1,m,1],
 [0,0,1,0]]

[[0,0,0,0],
 [0,1,0,1],
 [0,0,1,0],
 [0,1,0,1]]

[[0,1,0,1],
 [1,0,3,0],
 [0,3,1,3],
 [1,0,3,0]]

[[2,0,3,0],
 [0,4,1,3],
 [3,1,4,1],
 [0,3,1,2]]

# def recurse(n)
#   return 0 if n == 0
#   test = [0,3,1,2]
#   acc = test.length
#   test.each {|i| acc += recurse(n-1)}
#   acc
# end

def probabilityofalive(x, y, n, m: nil)
  return 0 if n == 0
  m ||= n
  positions = [[x-1,y],[x+1,y],[x,y-1],[x,y+1]]
  on_island = positions.reject do |point| 
    point.any? do |index|  
      index >= m || index < 0
    end
  end
  result = on_island.length
  on_island.each {|point|result += probabilityofalive(point[0], point[1],n-1, :m => m)}
  result
end

#puts recurse(2)
num = probabilityofalive(1, 1, 2)
# num = num / (4**2.to_f + 4**1)
puts num
