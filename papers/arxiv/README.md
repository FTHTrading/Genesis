# arXiv Submission Package

## Contents

| File | Description |
|------|-------------|
| `main.tex` | Primary manuscript (~690 lines LaTeX) |
| `references.bib` | BibTeX bibliography (6 entries) |
| `fig1_collapse_phase_transition.pdf` | Figure 1: Collapse phase transition diagram |

## Submission Category

**Primary**: cs.MA (Multi-Agent Systems)  
**Cross-list**: nlin.AO (Adaptation and Self-Organizing Systems), cs.CE (Computational Engineering)

## Building Locally

Requires a TeX distribution (TeX Live, MiKTeX, or MacTeX):

```bash
cd papers/arxiv
pdflatex main
bibtex main
pdflatex main
pdflatex main
```

Three passes are required: first for structure, `bibtex` for references, second and third for cross-references.

## Submitting to arXiv

1. Go to https://arxiv.org/submit
2. Upload all three files (`main.tex`, `references.bib`, `fig1_collapse_phase_transition.pdf`)
3. Select primary category: **cs.MA**
4. Add cross-list categories: **nlin.AO**, **cs.CE**
5. Paste the abstract from `main.tex`
6. Submit

arXiv compiles LaTeX server-side using TeX Live. No local compilation is required for submission.

## Pre-Submission Checklist

- [x] Title: Descriptive, no hype
- [x] Abstract: System → method → results with CIs → limitations
- [x] All confidence intervals: Clopper-Pearson exact
- [x] Power analysis included
- [x] Phase transition documented with figure
- [x] 11 limitations explicitly stated
- [x] Known failure modes catalogued (Appendix F)
- [x] Replication protocol with hash verification
- [x] Source code URL: https://github.com/FTHTrading/Genesis
- [x] No marketing language
- [x] No institutional narrative
- [x] Independent replication status: honestly disclosed as not yet performed
