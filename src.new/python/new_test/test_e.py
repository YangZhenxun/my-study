import edge_tts
import asyncio
from pydub import AudioSegment, playback

TEXT = "果果是个好孩子,可以拿5块钱！"
VOICE = "zh-CN-XiaoxiaoNeural"
OUTPUT_FILE = "test.mp3"


async def amain() -> None:
    """Main function"""
    communicate = edge_tts.Communicate(TEXT, VOICE)
    await communicate.save(OUTPUT_FILE)


if __name__ == "__main__":
    asyncio.run(amain())
    mp3f = AudioSegment.from_mp3(OUTPUT_FILE)
    while True:
        playback.play(mp3f)
