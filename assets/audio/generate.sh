ffmpeg -y -i roll-one.wav \
  -filter_complex "\
    [0:a]adelay=0,volume=1,asetrate=44100*1.0[a1];\
    [0:a]adelay=100,volume=1.1,asetrate=44100*1.2[a2];\
    [0:a]adelay=150,volume=0.8,asetrate=44100*0.8[a3];\
    [a1][a2][a3]amix=inputs=3:normalize=0\
  " \
  -acodec adpcm_ima_wav \
  roll-some.wav

ffmpeg -y -i roll-one.wav \
  -filter_complex "\
    [0:a]adelay=0,volume=1,asetrate=44100*1.0[a1];\
    [0:a]adelay=101,volume=1.1,asetrate=44100*1.2[a2];\
    [0:a]adelay=150,volume=0.8,asetrate=44100*0.8[a3];\
    [0:a]adelay=75,volume=0.6,asetrate=44100*0.7[a4];\
    [0:a]adelay=32,volume=1.2,asetrate=44100*1.3[a5];\
    [0:a]adelay=117,volume=0.9,asetrate=44100*1.1[a6];\
    [a1][a2][a3][a4][a5][a6]amix=inputs=6:normalize=0\
  "\
  -acodec adpcm_ima_wav \
  roll-many.wav
