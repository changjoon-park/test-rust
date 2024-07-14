import statistics


class Temperature:
    def __init__(self, celsius):
        self.celsius = celsius

    @classmethod
    def from_fahrenheit(cls, fahrenheit):
        celsius = (fahrenheit - 32.0) * 5.0 / 9.0
        return cls(celsius)

    def to_fahrenheit(self):
        return (self.celsius * 9.0 / 5.0) + 32.0

    def __str__(self):
        return f"{self.celsius:.1f}°C is {self.to_fahrenheit():.1f}°F"

    def __lt__(self, other):
        return self.celsius < other.celsius


def parse_temperature(input_str):
    input_str = input_str.strip()
    if input_str[-1].upper() == "C":
        try:
            return Temperature(float(input_str[:-1]))
        except ValueError:
            raise ValueError("Invalid format for Celsius temperature")
    elif input_str[-1].upper() == "F":
        try:
            return Temperature.from_fahrenheit(float(input_str[:-1]))
        except ValueError:
            raise ValueError("Invalid format for Fahrenheit temperature")
    else:
        raise ValueError("Temperature must end with 'C' or 'F'")


def main():
    print("Temperature Converter (Celsius to Fahrenheit and vice versa)")
    while True:
        print(
            "Enter temperatures with suffix 'C' for Celsius or 'F' for Fahrenheit separated by commas (or type 'exit' to quit):"
        )
        user_input = input().strip()
        if user_input.lower() == "exit":
            break

        temperatures = []
        for temp_str in user_input.split(","):
            try:
                temp = parse_temperature(temp_str)
                temperatures.append(temp)
            except ValueError as e:
                print(f"Error: {str(e)} for input '{temp_str.strip()}'")

        if not temperatures:
            print("No valid temperatures entered.")
            continue

        print("Valid temperatures:")
        for temp in temperatures:
            print(temp)

        avg_celsius = statistics.mean(temp.celsius for temp in temperatures)
        print(f"Average temperature: {avg_celsius:.1f}°C")

        if temperatures:
            min_temp = min(temperatures)
            max_temp = max(temperatures)
            print(f"Lowest temperature: {min_temp}")
            print(f"Highest temperature: {max_temp}")

        grouped = {"Celsius": [], "Fahrenheit": []}
        for temp in temperatures:
            if temp.to_fahrenheit() == temp.celsius:
                grouped["Celsius"].append(temp)
            else:
                grouped["Fahrenheit"].append(temp)

        for scale, temps in grouped.items():
            if temps:
                print(f"{scale} temperatures:")
                for t in temps:
                    print(f"  {t}")


if __name__ == "__main__":
    main()
