import time


def progress_bar(finish_tasks_number, tasks_number, complete_time):
    """
    进度条

    :param finish_tasks_number: int, 已完成的任务数
    :param tasks_number: int, 总的任务数
    :param complete_time: float, 已完成的任务所消耗的总时间
    :return:
    """

    percentage = round(finish_tasks_number / tasks_number * 100)
    finished_label = "▓" * (percentage // 2)
    unfinished_label = "-" * (100 - percentage)
    arrow = "->"
    if not finished_label or not unfinished_label:
        arrow = ""
    print("\r{}% [{}{}{}] {:.2f}s".format(percentage, finished_label, arrow, unfinished_label, complete_time), end="")


if __name__ == '__main__':
    start = time.perf_counter()
    for i in range(0, 101):
        duration = time.perf_counter() - start
        progress_bar(i, 100, duration)
        print("i")
