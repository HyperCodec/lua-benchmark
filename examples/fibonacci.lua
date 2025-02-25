function fibonacci(n)
    if n == 0 or n == 1 then
        return n
    end

    return fibonacci(n-1) + fibonacci(n-2)
end