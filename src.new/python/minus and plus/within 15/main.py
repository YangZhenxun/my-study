import random
import time
import sys
import os
import pyttsx3
import edge_tts
import asyncio
from pydub import AudioSegment
from pydub.playback import play

def __init__():
    main_dir = os.path.dirname(os.path.abspath(__file__))
    wrong_file = os.path.join(main_dir, "wrong.mp3")
    right_file = os.path.join(main_dir, "right.mp3")
    numget_file = os.path.join(main_dir, "num_get.mp3")
    progress_file = os.path.join(main_dir, "progress.mp3")
    minus_file = os.path.join(main_dir, "minus.mp3")
    plus_file = os.path.join(main_dir, "plus.mp3")
    start = time.perf_counter()
    engine = pyttsx3.init()
    a = 0
    err = 0
    source = 0
    no = 0
    lis = ["+", "-"]
    engine.setProperty('volume',0.8)
    while True:
        rand_a = random.randint(1, 15)
        rand_b = random.randint(1, 15)
        random_flag = random.randint(0, 1)
        if lis[random_flag] == "-":
            if rand_a >= rand_b:
                try:
                    async def amain() -> None:
                        communicate = edge_tts.Communicate(f"{rand_a}减{rand_b}=", "zh-CN-XiaoxiaoNeural")
                        await communicate.save(minus_file)
                    asyncio.run(amain())
                    progress = AudioSegment.from_mp3(minus_file)
                    play(progress)
                    os.remove(minus_file)
                    del amain
                    inp = int(input(f"{rand_a}-{rand_b}="))
                except:
                    print("请输入数字！")
                    num_get = AudioSegment.from_mp3(numget_file)
                    play(num_get)
                    no += 1
                    a += 1
                    print(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%")
                    async def amain() -> None:
                        communicate = edge_tts.Communicate(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%", "zh-CN-XiaoxiaoNeural")
                        await communicate.save(progress_file)
                    asyncio.run(amain())
                    progress = AudioSegment.from_mp3(progress_file)
                    play(progress)
                    os.remove(progress_file)
                    del amain
                    continue
                maths = rand_a - rand_b
                if inp == maths:
                    print("你答对了！")
                    right = AudioSegment.from_mp3(right_file)
                    play(right)
                    source += 1
                else:
                    print(f"你答错了！\n正确答案是{maths}!")
                    async def amain() -> None:
                        communicate = edge_tts.Communicate(f"你答错了！\n正确答案是{maths}!", "zh-CN-XiaoxiaoNeural")
                        await communicate.save(wrong_file)
                    asyncio.run(amain())
                    wrong = AudioSegment.from_mp3(wrong_file)
                    play(wrong)
                    os.remove(wrong_file)
                    del amain
                    err += 1
                    source -= 1
                a += 1
                now = time.perf_counter()
                print(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{(a - err - no)/a * 100:.2f}%")
                async def amain() -> None:
                    communicate = edge_tts.Communicate(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{(a - err - no)/a * 100:.2f}%", "zh-CN-XiaoxiaoNeural")
                    await communicate.save(progress_file)
                asyncio.run(amain())
                progress = AudioSegment.from_mp3(progress_file)
                play(progress)
                os.remove(progress_file)
                del amain
            else:
                continue
        elif lis[random_flag] == "+":
            rand_a = random.randint(0, 5)
            rand_b = random.randint(0, 5)
            try:
                async def amain() -> None:
                    communicate = edge_tts.Communicate(f"{rand_a}+{rand_b}=", "zh-CN-XiaoxiaoNeural")
                    await communicate.save(plus_file)
                asyncio.run(amain())
                progress = AudioSegment.from_mp3(plus_file)
                play(progress)
                os.remove(plus_file)
                del amain
                inp = int(input(f"{rand_a}+{rand_b}="))
            except:
                print("请输入数字！")
                num_get = AudioSegment.from_mp3(numget_file)
                play(num_get)
                no += 1
                a += 1
                now = time.perf_counter()
                print(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%")
                async def amain() -> None:
                    communicate = edge_tts.Communicate(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%", "zh-CN-XiaoxiaoNeural")
                    await communicate.save(progress_file)
                asyncio.run(amain())
                progress = AudioSegment.from_mp3(progress_file)
                play(progress)
                os.remove(progress_file)
                del amain
                continue
            maths = rand_a + rand_b
            if inp == maths:
                print("你答对了！")
                right = AudioSegment.from_mp3(right_file)
                play(right)
                source += 1
            else:
                print(f"你答错了！\n正确答案是{maths}!")
                async def amain() -> None:
                    communicate = edge_tts.Communicate(f"你答错了！\n正确答案是{maths}!", "zh-CN-XiaoxiaoNeural")
                    await communicate.save(wrong_file)
                asyncio.run(amain())
                wrong = AudioSegment.from_mp3(wrong_file)
                play(wrong)
                os.remove(wrong_file)
                del amain
                err += 1
                source -= 1

            a += 1
            now = time.perf_counter()
            print(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%")
            async def amain() -> None:
                communicate = edge_tts.Communicate(f"共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，正确率：{((a - err - no)/a * 100):.2f}%", "zh-CN-XiaoxiaoNeural")
                await communicate.save(progress_file)
            asyncio.run(amain())
            progress = AudioSegment.from_mp3(progress_file)
            play(progress)
            os.remove(progress_file)
            del amain

if __name__ == "__main__":
    __init__()
