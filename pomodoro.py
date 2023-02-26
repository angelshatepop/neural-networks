import sys, time
from getch import getch

def focus(minutes):
    seconds = 60
    for i in range(minutes*60-1, 0, -1):
        if i <= 0:
            break
        else:
            seconds -= 1
            mins =  int(i/60)
            if mins < 10 and seconds == 0:
                sys.stdout.write("\rfocus...  {}:00".format(int(mins)))
                sys.stdout.flush()
                seconds = 60
            elif mins < 10 and seconds > 0:
                if seconds < 10:
                    sys.stdout.write("\rfocus...  {}:0{}".format(int(mins), seconds))
                    sys.stdout.flush()
                else:
                    sys.stdout.write("\rfocus...  {}:{}".format(int(mins), seconds))
                    sys.stdout.flush()
            elif mins >= 10 and seconds == 0:
                sys.stdout.write("\rfocus... {}:00".format(int(mins)))
                sys.stdout.flush()
                seconds = 60
            else:
                if seconds < 10:
                    sys.stdout.write("\rfocus... {}:0{}".format(int(mins), seconds))
                    sys.stdout.flush()
                else:
                    sys.stdout.write("\rfocus... {}:{}".format(int(mins), seconds))
                    sys.stdout.flush()
            time.sleep(1)
def rest(minutes):
    seconds = 60
    for i in range(minutes*60-1, 0, -1):
        if i <= 0:
            break
        else:
            seconds -= 1
            mins =  int(i/60)
            if mins < 10 and seconds == 0:
                sys.stdout.write("\rrest...   {}:00".format(int(mins)))
                sys.stdout.flush()
                seconds = 60
            elif mins < 10 and seconds > 0:
                if seconds < 10:
                    sys.stdout.write("\rrest...   {}:0{}".format(int(mins), seconds))
                    sys.stdout.flush()
                else:
                    sys.stdout.write("\rrest...   {}:{}".format(int(mins), seconds))
                    sys.stdout.flush()
            elif mins >= 10 and seconds == 0:
                sys.stdout.write("\rrest...  {}:00".format(int(mins)))
                sys.stdout.flush()
                seconds = 60
            else:
                if seconds < 10:
                    sys.stdout.write("\rrest...  {}:0{}".format(int(mins), seconds))
                    sys.stdout.flush()
                else:
                    sys.stdout.write("\rrest...  {}:{}".format(int(mins), seconds))
                    sys.stdout.flush()
            time.sleep(1)

def pomodoro(cycles):
    while cycles != 0:
        focus(25)
        rest(5)
        cycles -= 1
        sys.stdout.write("\rrest...  pomodoro cycle complete\n")
        sys.stdout.flush()

pomodoro(4)