from enum import Enum


class Action(Enum):
    WALK = (1, "Walk", "Walking")
    RUN = (2, "Run", "Running")
    END = (0, "End", "Ending")

    @classmethod
    def _missing_(cls, value):
        for item in cls:
            if item.value[0] == value:
                return item
        return super()._missing_(value)

    def __str__(self):
        return f"{self.value[0]}: {self.value[1]}"


def loop():
    while True:
        print(", ".join(str(a) for a in Action))

        try:
            num = Action(int(input("> ")))
        except ValueError:
            print("Invalid input")
            continue

        match num:
            case Action.WALK:
                print(Action.WALK.value[2])
            case Action.RUN:
                print(Action.RUN.value[2])
            case Action.END:
                break


if __name__ == "__main__":
    loop()
