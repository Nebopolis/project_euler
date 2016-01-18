# Pots of gold game: Two players A & B. There are pots of gold arranged in a line, each containing some gold coins (the players can see how many coins are there in each gold pot - perfect information). They get alternating turns in which the player can pick a pot from one of the ends of the line. The winner is the player which has a higher number of coins at the end. The objective is to "maximize" the number of coins collected by A, assuming B also plays optimally. A starts the game.

# The idea is to find an optimal strategy that makes A win knowing that B is playing optimally as well. How would you do that? 

# At the end I was asked to code this strategy!



def best_result(list)
  return 0 if list.length == 0
  return list[0] if list.length == 1
  no_last = list.take(list.length - 1)
  no_first = list.drop(1)
  a = list.first + [best_result(no_first.drop(1)), best_result(no_first.take(no_first.length-1))].min
  b = list.last + [best_result(no_last.drop(1)), best_result(no_last.take(no_last.length-1))].min
  [a,b].max
end
list = [1,3,4,51,23,4,512,3,532]
puts best_result(list)

