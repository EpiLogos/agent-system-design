import { describe, expect, it } from 'vitest'
import { chronologicalSpans, nextSpanRef, orderedTraces, waterfallGeometry } from './read-model'
import { sssfParityTrace } from './fixtures/sssf-parity'

describe('execution trace read model', () => {
  it('orders phases chronologically while retaining queued detail last', () => {
    const spans = chronologicalSpans(sssfParityTrace)
    expect(spans.map((span) => span.name)).toEqual(['request', 'plan', 'build', 'test', 'review'])
  })

  it('ports the SSSF readable-block geometry without overlaps', () => {
    const geometry = waterfallGeometry(sssfParityTrace, Date.parse('2026-08-16T20:00:18.000Z'))
    expect(geometry[0]).toMatchObject({ spanRef: 'sssf-span:p-request', leftPct: 0.4 })
    const postRequest = geometry.slice(1)
    for (const item of postRequest) expect(item.widthPct).toBeGreaterThanOrEqual(3.4)
    for (let i = 1; i < postRequest.length; i += 1) {
      const previous = postRequest[i - 1]!
      const current = postRequest[i]!
      expect(current.leftPct + 0.001).toBeGreaterThanOrEqual(previous.leftPct + previous.widthPct)
    }
    expect(geometry.every((item) => item.leftPct >= 0 && item.leftPct + item.widthPct <= 100.01)).toBe(true)
  })

  it('supports deterministic keyboard-relative selection', () => {
    expect(nextSpanRef(sssfParityTrace, undefined, 1)).toBe('sssf-span:p-request')
    expect(nextSpanRef(sssfParityTrace, 'sssf-span:p-request', 1)).toBe('sssf-span:p-plan')
    expect(nextSpanRef(sssfParityTrace, 'sssf-span:p-plan', -1)).toBe('sssf-span:p-request')
  })

  it('orders execution cards newest first', () => {
    const older = { ...sssfParityTrace, executionRef: 'execution:older', startedAt: '2026-08-16T19:00:00.000Z' }
    expect(orderedTraces([older, sssfParityTrace])[0]?.executionRef).toBe(sssfParityTrace.executionRef)
  })
})
