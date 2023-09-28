# frozen_string_literal: true

# Class do the scaffolding for advent of advent_of_code
# It checks the last day and create a folder with file for the next day
class AocScaff
  YEAR = '2020'

  def self.scaff_next_day
    check_pwd
    next_day = find_next_day_folder

    next_day_dir_path = File.join(Dir.pwd, YEAR, next_day)

    Dir.mkdir(next_day_dir_path)

    rb_file = File.join(next_day_dir_path, 'adv.rb')
    input_file = File.join(next_day_dir_path, 'input.txt')

    File.new(rb_file, 'w')
    puts "Created file: #{rb_file}"
    File.new(input_file, 'w')
    puts "Created file: #{input_file}"
  end

  private_class_method def self.check_pwd
    pwd = Dir.pwd
    current_dir = File.basename(pwd)
    parent_dir = File.basename(File.dirname(pwd))

    return unless current_dir != 'ruby' || parent_dir != 'advent_of_code'

    raise "Wrong directory. Expected to be in '../advent_of_code/ruby'"
  end

  private_class_method def self.find_next_day_folder
    # For now I'm doing the year 2020 only...
    # When I get to make another one I would add cmd arguments

    day_folders = Dir.glob(File.join(Dir.pwd, YEAR, '*'))
                     .select { |entry| File.directory?(entry) }
                     .map { |path| File.basename(path) }
    day_folders.sort

    last_day = day_folders.last.to_i || 0

    raise "Day can't be bigger than 25" if last_day >= 25

    format('%02d', (last_day + 1))
  end
end
