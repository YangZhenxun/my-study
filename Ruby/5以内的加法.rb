while true
  n1 = rand(1..5)
  n2 = rand(1..5)
  maths = n1 + n2
  print "#{n1}+#{n2}="
  a = gets.to_i
  if a == maths
    puts "你答对了!"
  else
    puts "你答错了!\n正确答案是#{maths}"
  end
end
