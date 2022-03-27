#!/usr/bin/env python3
# Copyright(c) 2022 Blacknon. All rights reserved.
# Use of this source code is governed by an MIT license
# that can be found in the LICENSE file.
# -*- coding: utf-8 -*-

import os
import json


def create_dict(data: list):
    result = list()
    for d in data:
        line = dict()
        line['data'] = d

        result.append(line)

    return result


def unique_array(data: list):
    result = []
    for d in data:
        line = list(set(d))
        result.append(line)

    return result


def merge_array(data1: list, data2: list):
    result = []

    count_data1 = 0
    for data_element1 in data1:
        for element_char1 in data_element1:
            count_data2 = 0
            for data_element2 in data2:
                if element_char1 in data_element2:
                    result.append(data_element1 + data_element2)
                    del data2[count_data2]
                    del data1[count_data1]
                    break
                count_data2 += 1

        count_data1 += 1

    result.extend(data1)
    result.extend(data2)

    return result


def create_charstr(text: str):
    result = ""
    split_text = text.split(" ")
    for t in split_text:
        result = result + chr(int(t, 16))

    return result


def gen_from_original():
    """
    """
    # get file path
    current_dir = os.path.dirname(__file__)
    filename = 'original_char_codes.txt'
    filepath = os.path.join(current_dir, filename)

    # open file
    with open(filepath) as f:
        # create result
        result = []

        lines = f.readlines()
        for line in lines:
            result_line = []

            # skip comment out line
            if line.startswith('#'):
                continue

            # strip linebreak
            line = line.strip()

            split_line = line.split(',')
            for char in split_line:
                c = chr(int(char, 16))
                result_line.append(c)

            result.append(result_line)

    return result


def gen_from_codebox_homoglyph():
    """
    """
    # get file path
    current_dir = os.path.dirname(__file__)
    filename = 'char_codes.txt'
    filepath = os.path.join(current_dir, filename)

    # open file
    with open(filepath) as f:
        # create result
        result = []

        lines = f.readlines()
        for line in lines:
            result_line = []

            # skip comment out line
            if line.startswith('#'):
                continue

            # strip linebreak
            line = line.strip()

            split_line = line.split(',')
            for char in split_line:
                c = chr(int(char, 16))
                result_line.append(c)

            result.append(result_line)

    return result


def gen_from_unicode_org_confusables():
    """
    """
    # get file path
    current_dir = os.path.dirname(__file__)
    filename = 'confusables.txt'
    filepath = os.path.join(current_dir, filename)

    # open file
    with open(filepath) as f:
        # create tmp
        tmp = dict()

        lines = f.readlines()
        for line in lines:
            # strip bom
            line = line.lstrip("\ufeff")

            # strip linebreak
            line = line.strip()

            # skip comment out line
            if line.startswith('#'):
                continue

            if len(line) == 0:
                continue

            # split line
            split_line = line.split(';\t')

            # get value
            key = split_line[1].strip(' ')
            value = split_line[0].strip(' ')

            # create str
            key_str = create_charstr(key)
            velue_str = create_charstr(value)

            if key_str not in tmp:
                tmp[key_str] = []

            tmp[key_str].append(velue_str)

    # tmp(dict) to result(array)
    result = []
    for key, value in tmp.items():
        line = []
        line.append(key)
        line.extend(value)

        result.append(line)

    return result


def main():
    # get codebox_homoglyph
    codebox_homoglyph = gen_from_codebox_homoglyph()
    print("codebox_homoglyph: ", len(codebox_homoglyph))

    original = gen_from_original()
    print("original: ", len(original))

    # get unicode_org_confusables
    unicode_org_confusables = gen_from_unicode_org_confusables()
    print("unicode_org_confusables: ", len(unicode_org_confusables))

    data = merge_array(codebox_homoglyph, original)
    print("data: ", len(data))
    data = merge_array(data, unicode_org_confusables)
    print("data: ", len(data))

    data = unique_array(data)

    print("data: ", len(data))

    result_data = create_dict(data)

    current_dir = os.path.dirname(__file__)
    filename = 'homoglyph.json'
    filepath = os.path.join(current_dir, filename)
    with open(filepath, 'w') as f:
        json.dump(result_data, f, indent=4)


if __name__ == '__main__':
    main()
