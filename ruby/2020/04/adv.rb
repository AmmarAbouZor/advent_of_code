# frozen_string_literal: true

require '../../aoc_base'

# Extend string with num? method to check if the string is basic number
class String
  def num?
    self =~ /\A[0-9]+\Z/
  end
end

# Year 2020 Day 04
class Aoc04 < AocBase
  REQ_KEYS = %w[byr iyr eyr hgt hcl ecl pid].freeze
  def parse_input(input)
    pass_dict = input.split("\n\n")
    pass_dict.map { |pass| parse_pass(pass) }
  end

  def parse_pass(pass_text)
    pass_dict = {}
    pass_text.lines.each do |line|
      line.split(' ').each do |pair|
        key, value = pair.split(':')
        pass_dict[key] = value
      end
    end

    pass_dict
  end

  def check_pass_has_required(pass_dict)
    REQ_KEYS.all? { |key| pass_dict.key?(key) }
  end

  def validate_pass(pass_dict)
    check_birth_year(pass_dict) &&
      check_issue_year(pass_dict) &&
      check_exp_year(pass_dict) &&
      check_height(pass_dict) &&
      check_hair_color(pass_dict) &&
      check_eye_color(pass_dict) &&
      check_pass_id(pass_dict)
  end

  def check_birth_year(pass_dict)
    value = pass_dict['byr']
    return false if value.nil?

    value.num? && (1920..2002).include?(value.to_i)
  end

  def check_issue_year(pass_dict)
    value = pass_dict['iyr']
    return false if value.nil?

    value.num? && (2010..2020).include?(value.to_i)
  end

  def check_exp_year(pass_dict)
    value = pass_dict['eyr']
    return false if value.nil?

    value.num? && (2020..2030).include?(value.to_i)
  end

  def check_height(pass_dict)
    value = pass_dict['hgt']
    return false if value.nil?

    unit = value[-2..]
    num = value[..-3]
    if unit == 'cm'
      num.num? && (150..193).include?(num.to_i)
    elsif unit == 'in'
      num.num? && (59..76).include?(num.to_i)
    else
      false
    end
  end

  def check_hair_color(pass_dict)
    value = pass_dict['hcl']
    return false if value.nil?

    value =~ /^#[0-9a-f]{6}$/
  end

  def check_eye_color(pass_dict)
    value = pass_dict['ecl']
    return false if value.nil?

    valid_colors = %w[amb blu brn gry grn hzl oth]
    valid_colors.include?(value)
  end

  def check_pass_id(pass_dict)
    value = pass_dict['pid']
    return false if value.nil?

    value =~ /^[0-9]{9}$/
  end

  def passes_with_required(input)
    pass_dicts = parse_input(input)
    pass_dicts.count { |pass_dict| check_pass_has_required(pass_dict) }
  end

  def passes_with_validation(input)
    pass_dicts = parse_input(input)
    pass_dicts.count { |pass_dict| validate_pass(pass_dict) }
  end

  def part_one
    passes_with_required(@input)
  end

  def part_two
    passes_with_validation(@input)
  end

  def do_tests
    assert_equal passes_with_required(@test_input), 2
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc04.new
  aoc.run
end
