# Echelon: Your AI Academic Advisor

Echelon is an open-source AI platform designed to augment academic advising through Retrieval-Augmented Generation (RAG) and Large Language Models (LLMs). It provides students with personalized, real-time, and document-grounded answers based on their academic records, university course catalogs, and internal documents.

> 🔗 [Live Demo](https://echelongpt.com) | 🖋️ [Thesis Paper (PDF)](link-to-thesis-if-public) | 📧 Contact: [harrison.leath@tcu.edu](mailto:harrison.leath@tcu.edu)

> **Disclaimer:** Demo hosted at [echelongpt.com](https://echelongpt.com) is live until the end of Spring 2025.

![Architecture Overview](link-to-architecture-diagram-if-public)

---

# ✨ Features

- **Semantic Search**: Fast, high-precision retrieval with BGE-large-en-v1.5 and Qdrant.
- **Personalized Advising**: Vision-language models parse transcripts into structured academic profiles.
- **Grounded Responses**: Combines user queries, retrieved context, and user academic data.
- **High-Concurrency Backend**: Built in Rust with Axum and Tokio for low-latency performance.
- **Modern UI**: Built with SvelteKit, TypeScript, TailwindCSS, and shadcn/ui.
- **Scalable Deployment**: Docker-based deployment with support for GPU-backed inference.
- **Privacy-First**: Student data remains under institutional control.

---

# 🌐 System Architecture

| Component | Technologies |
|-----------|--------------|
| **Frontend** | SvelteKit + TypeScript + TailwindCSS + shadcn/ui |
| **Backend** | Rust + Axum + Tokio |
| **Compute** | Ollama (for LLM inference, containerized) |
| **Storage** | PostgreSQL, Qdrant, MinIO |

![System Flow](link-to-system-flow-diagram)

### High-Level Workflow

1. **Transcript Upload**: Students upload an unofficial transcript.
2. **Profile Parsing**: Vision-language model extracts structured academic data.
3. **Question Embedding**: User queries embedded into vector space.
4. **Context Retrieval**: Semantic search retrieves relevant documents and user academic profile.
5. **LLM Generation**: Personalized, grounded responses generated and streamed back.

---

# 🚀 Quickstart

### Prerequisites
- Docker & Docker Compose
- Node.js (for frontend development)
- Rust (for backend development)

### Clone the Repo
```bash
git clone https://github.com/leathalman/echelon.git
cd echelon
```

### Environment Setup
```bash
cp .env.example .env
# Fill in values for database credentials, API endpoints, etc.
```

### Build & Run Locally
```bash
# Build and run backend/frontend/databases
sudo docker-compose up --build

# (Optional) Build frontend separately
cd frontend
npm install
npm run dev
```

---

# 🎓 Academic Personalization

Echelon uses multimodal models (like Qwen 2.5 VL 72B) to parse transcripts and generate academic profiles. 
These profiles are used dynamically during inference to:

- Evaluate graduation readiness
- Suggest remaining major courses
- Identify academic strengths and areas of improvement

### Example Student Questions
- "Am I eligible to graduate next semester?"
- "Which upper-level COSC electives have I not completed yet?"
- "What GPA do I need to graduate with honors?"

---

# 🌍 Deployment Overview

Echelon is infrastructure-agnostic and can be deployed on:
- Internal university servers
- DigitalOcean / AWS / Azure
- Runpod for scalable GPU inference

**Key Deployment Details**:
- Nginx for TLS termination
- Docker Compose orchestration
- PostgreSQL managed instance
- Qdrant for vector search
- Ollama server for LLM inference


---

# 🚧 Future Roadmap

- [ ] Hybrid search combining dense and sparse retrieval
- [ ] Inline citations for transparency
- [ ] Proactive advising based on student trajectory
- [ ] Full automation of transcript parsing (awaiting Ollama vision model support)
- [ ] Expand to non-academic departments (Admissions, Athletics, IT, etc.)

---

# 📅 Built With

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum/latest/axum/)
- [Tokio](https://tokio.rs/)
- [SvelteKit](https://kit.svelte.dev/)
- [TailwindCSS](https://tailwindcss.com/)
- [Qdrant](https://qdrant.tech/)
- [PostgreSQL](https://www.postgresql.org/)
- [MinIO](https://min.io/)
- [Ollama](https://ollama.com/)

---

# 👤 Author

- Harrison Leath  
- Email: [harrison.leath@tcu.edu](mailto:harrison.leath@tcu.edu)
- GitHub: [@leathalman](https://github.com/leathalman)

---

# 📖 Thesis Reference

If you would like a full technical deep dive into Echelon's architecture and rationale, see the [full thesis paper here](link-to-thesis).

---

# 🌐 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

# 🌟 Acknowledgements

- [Word2Vec](https://arxiv.org/abs/1301.3781)
- [Sentence-BERT](https://arxiv.org/abs/1908.10084)
- [BGE-large-en-v1.5](https://arxiv.org/abs/2502.13923)
- [Mistral Small 24B](https://huggingface.co/mistralai/Mistral-Small-24B-Instruct-2501)
- [Qwen 2.5 VL](https://arxiv.org/abs/2502.13923)
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
