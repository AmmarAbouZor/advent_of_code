require './scripts/scaff'

task :scaff do
  AocScaff.scaff_next_day(true)
end

task :scaff_no_test do
  AocScaff.scaff_next_day(false)
end

task :last_day do
  AocScaff.last_day_path
end
