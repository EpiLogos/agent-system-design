import json
import sys


def emit(value):
    sys.stdout.write(json.dumps(value, default=str))
    sys.stdout.flush()


def usage_dict(result):
    try:
        usage = result.usage() if callable(getattr(result, 'usage', None)) else getattr(result, 'usage', None)
    except Exception:
        usage = None
    if usage is None:
        return None
    return {
        'input_tokens': int(getattr(usage, 'input_tokens', 0) or 0),
        'output_tokens': int(getattr(usage, 'output_tokens', 0) or 0),
        'total_tokens': int(getattr(usage, 'total_tokens', 0) or 0),
    }


def main():
    payload = json.load(sys.stdin)
    operation = payload.get('operation')
    model = payload.get('model')
    if not model or model.endswith(':'):
        emit({'ready': False, 'error': 'A concrete Pydantic AI model id is required.'})
        return 2

    try:
        from pydantic_ai import Agent
    except Exception as exc:
        emit({'ready': False, 'error': f'Real pydantic_ai import failed: {exc}'})
        return 3

    if operation == 'preflight':
        try:
            Agent(model)
            emit({'ready': True, 'model': model, 'framework': 'pydantic-ai'})
            return 0
        except Exception as exc:
            emit({'ready': False, 'error': f'Pydantic model construction failed: {exc}'})
            return 4

    if operation != 'complete':
        emit({'ready': False, 'error': f'Unknown operation {operation!r}'})
        return 5

    try:
        agent = Agent(model, system_prompt=payload.get('system') or '')
        result = agent.run_sync(payload.get('prompt') or '')
        output = result.output if hasattr(result, 'output') else getattr(result, 'data', '')
        model_name = None
        try:
            messages = result.all_messages()
            if messages:
                model_name = getattr(messages[-1], 'model_name', None)
        except Exception:
            pass
        emit({
            'ready': True,
            'output': output if isinstance(output, str) else json.dumps(output, default=str),
            'usage': usage_dict(result),
            'model_name': model_name or model,
        })
        return 0
    except Exception as exc:
        emit({'ready': False, 'error': f'Pydantic live completion failed: {exc}'})
        return 6


if __name__ == '__main__':
    raise SystemExit(main())
