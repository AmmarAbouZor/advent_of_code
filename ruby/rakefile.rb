require './scripts/scaff'

task :scaff do
  AocScaff.scaff_next_day
end

task :last_day do
  AocScaff.last_day_path
end
