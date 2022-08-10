puts "请输入您的身高(m):"
hei = gets.to_i
puts "请输入您的体重(kg):"
kg = gets.to_i
BMI = hei/(kg*kg)
if BMI <= 0
  puts "你的体重是0"
