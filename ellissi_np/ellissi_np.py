import csv
from itertools import groupby
from functools import reduce
import math


class Answer:
    def __init__(self, i: int, y: int, m: int, s: str, n: int, t_v: bool):
        self.ID = i
        self.Year = y
        self.Month = m
        self.Sex = s
        self.Number = n
        self.Truth_Value = t_v

    def __str__(self):
        return f"{self.Year};{self.Month}, {self.Number}, {self.Truth_Value}"


class Question:
    def __init__(self, i_t: str, s: str, w_o: str, n: int, t_v: bool):
        self.Idenity_Type = i_t
        self.Structure = s
        self.Word_Order = w_o
        self.Number = n
        self.Truth_Value = t_v

    def __str__(self):
        return f"Question({self.Idenity_Type}, {self.Structure}, {self.Word_Order}, {self.Number}, {self.Truth_Value})"


answers: list[Answer] = []
questions = []


# Open the CSV file
with open("answer.csv", "r") as csv_file:
    csv_reader = csv.reader(csv_file)

    # Skip the header row
    next(csv_reader)
    i = 1

    # Read and process the data
    for row in csv_reader:
        y, m = 0, 0
        if ";" in row[2]:
            y, m = row[2].split(";")
            y, m = int(y), int(m)
        else:
            y = int(row[2])

        s = row[4]
        n = int(row[12])
        t_v = row[13] == "T"
        answers.append(Answer(i, y, m, s, n, t_v))
        if n == 24:
            i += 1

# answers is now populated


with open("question.csv", "r") as csv_file:
    csv_reader = csv.reader(csv_file)

    # Skip the header row
    next(csv_reader)

    # Read and process the data
    for row in csv_reader:
        i_t = row[2]
        s = row[3]
        w_o = row[4]
        n = int(row[7])
        t_v = row[8] == "T"
        questions.append(Question(i_t, s, w_o, n, t_v))

# questions is now populated

# Now we can start the analysis

# answers: le risposte dei bambini
# questions: le domande


def filter(qs, anss, f):
    qs = [q.Number for q in qs if f(q)]
    return [ans for ans in anss if ans.Number in qs]


def group_by(anss, f):
    # Sort the list by the attribute you want to group by (Year in this case)
    # answers.sort(key=lambda answer: answer.Year)
    anss.sort(key=f)

    # Group the answers by the Year attribute
    grouped_answers = {}
    for year, group in groupby(anss, f):
        grouped_answers[year] = list(group)
    return grouped_answers


# a_1 = filter(questions, answers, lambda q: q.Idenity_Type == "Identity")

question_4 = filter(questions, answers, lambda q: q.Number == 4)  # 4, 8, 11
question_8 = filter(questions, answers, lambda q: q.Number == 8)
question_11 = filter(questions, answers, lambda q: q.Number == 11)


def var(qs, anss):
    a_2 = group_by(anss, lambda a: a.ID)

    # for key in a_2:
    #    print(key, len(a_2[key]))

    correctness = []

    for key in a_2:
        correctness.append(
            [
                x.Truth_Value == q.Truth_Value
                for x in a_2[key]
                for q in qs
                if x.Number == q.Number
            ]
        )

    # for i in correctness:
    #    for j in i:
    #        if j:
    #            print("Correct")
    #        else:
    #            print("Incorrect")
    #    print()

    correct_ans = []
    for i in correctness:
        correct_ans.append(reduce(lambda x, y: x + y, i))
    #    print(correct_ans[-1])

    # for i in correct_ans:
    #    print(i / len(correctness[0])) # % di risposte corrette

    media = reduce(lambda x, y: x + y, correct_ans) / len(correct_ans)
    deviazione = reduce(lambda x, y: x + y, [(i - media) ** 2 for i in correct_ans]) / (
        len(correct_ans) - 1
    )
    return (media, deviazione)


# self.Idenity_Type = i_t
#        self.Structure = s
#        self.Word_Order = w_o
#        self.Number = n
#        self.Truth_Value = t_v

# print(mid_var(identity, answers))
# print(mid_var(non_identity, answers))


def print_group(qs, ans, type):
    g2 = group_by(qs, type)
    res = ""
    for key in g2:
        m, v = var(g2[key], ans)
        print(
            key, ":", format(m, ".2f"), "on", len(g2[key]), "; var:", format(v, ".2f")
        )
        res += f"{key},{format(m, '.2f')},{len(g2[key])},{format(v, '.2f')}\n"

    with open("results.csv", "a") as f:
        f.write(res)


true = [q for q in questions if q.Truth_Value]
false = [q for q in questions if not q.Truth_Value]


print_group(questions, answers, lambda q: q.Structure)
print_group(questions, answers, lambda q: q.Word_Order)
print()

print("True")
print_group(true, answers, lambda q: q.Structure)
print_group(true, answers, lambda q: q.Word_Order)
print()

print("False")
print_group(false, answers, lambda q: q.Structure)
print_group(false, answers, lambda q: q.Word_Order)
