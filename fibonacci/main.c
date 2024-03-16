#include <stdio.h>

int	ft_isdigit(int c)
{
	if (c >= 48 && c <= 57)
		return (1);
	return (0);
}

int	ft_atoi(const char *str)
{
	int		sign;
	long	ret;

	ret = 0;
	sign = 1;
	while (*str == ' ' || *str == '\t' || *str == '\n' || *str == '\v'
		|| *str == '\f' || *str == '\r')
		++str;
	if (*str == '+' || *str == '-')
		if (*(str++) == '-')
			sign *= -1;
	while (ft_isdigit(*str))
		ret = ret * 10 + sign * (*str++ - '0');
	if (ret > 2147483647)
		return (-1);
	else if (ret < -2147483648)
		return (0);
	return ((int)ret);
}

int ft_fibonacci(int number)
{
	if (number == 0)
		return (0);
	if (number == 1)
		return (1);
	return (ft_fibonacci(number - 1) + ft_fibonacci(number - 2));
}

int main(int argc, char **argv)
{
	int number = ft_atoi(argv[1]);
	printf("%d", ft_fibonacci(number));
	return (0);
}
