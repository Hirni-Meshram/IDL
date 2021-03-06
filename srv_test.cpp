#include "srv_test.h"

using namespace RoseCommon;
using namespace RoseCommon::Packet;


SrvTest::Pote& SrvTest::Pote::set_a(const uint8_t a) {
    this->data.a = a;
    return *this;
}

uint8_t SrvTest::Pote::get_a() const {
    return data.a;
}

SrvTest::Pote& SrvTest::Pote::set_b(const uint8_t b) {
    this->data.b = b;
    return *this;
}

uint8_t SrvTest::Pote::get_b() const {
    return data.b;
}

SrvTest::Pote& SrvTest::Pote::set_c(const uint8_t c) {
    this->data.c = c;
    return *this;
}

uint8_t SrvTest::Pote::get_c() const {
    return data.c;
}

bool SrvTest::Pote::write(CRoseBasePolicy& writer) const {
    if (!writer.set_uint8_t(data.c)) {
        return false;
    }
    return true;
}

bool SrvTest::Pote::read(CRoseReader& reader) {
    if (!reader.get_uint8_t(data.c)) {
        return false;
    }
    return true;
}

constexpr size_t SrvTest::Pote::size() {
    return sizeof(data);
}

SrvTest::Pote2& SrvTest::Pote2::set_a(const uint8_t a) {
    for (size_t i = 0; i < 1; ++i) {
        this->bitset3[i + 0] = a & (1 << i);
    }
    return *this;
}

uint8_t SrvTest::Pote2::get_a() const {
    uint8_t a_tmp = 0;
    for (size_t i = 0; i < 1; ++i) {
        a_tmp |= (this->bitset3[i + 0] << i);
    }
    return a_tmp;
}

SrvTest::Pote2& SrvTest::Pote2::set_b(const uint8_t b) {
    for (size_t i = 0; i < 7; ++i) {
        this->bitset3[i + 1] = b & (1 << i);
    }
    return *this;
}

uint8_t SrvTest::Pote2::get_b() const {
    uint8_t b_tmp = 0;
    for (size_t i = 0; i < 7; ++i) {
        b_tmp |= (this->bitset3[i + 1] << i);
    }
    return b_tmp;
}

bool SrvTest::Pote2::write(CRoseBasePolicy& writer) const {
    if (!writer.set_bitset(bitset3)) {
        return false;
    }
    return true;
}

bool SrvTest::Pote2::read(CRoseReader& reader) {
    if (!reader.get_bitset(bitset3)) {
        return false;
    }
    return true;
}

constexpr size_t SrvTest::Pote2::size() {
    size_t size = 0;
    size += 8 / 8; // bitset3
    return size;
}


SrvTest::SrvTest() : CRosePacket(SrvTest::PACKET_ID) {
    set_server_packet();
}

SrvTest::SrvTest(CRoseReader reader) : CRosePacket(reader) {
    set_server_packet();
    
    if (!reader.get_bitset(bitset1)) {
        return false;
    }
    if (!reader.get_uint32_t(c)) {
        return false;
    }
    if (!reader.get_bitset(bitset2)) {
        return false;
    }
    if (!reader.get_iserialize(pote)) {
        return false;
    }
    if (!reader.get_iserialize(pote2)) {
        return false;
    }
}

SrvTest& SrvTest::set_a(const uint32_t a) {
    for (size_t i = 0; i < 3; ++i) {
        this->bitset1[i + 0] = a & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_a() const {
    uint32_t a_tmp = 0;
    for (size_t i = 0; i < 3; ++i) {
        a_tmp |= (this->bitset1[i + 0] << i);
    }
    return a_tmp;
}

SrvTest& SrvTest::set_b(const uint32_t b) {
    for (size_t i = 0; i < 5; ++i) {
        this->bitset1[i + 3] = b & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_b() const {
    uint32_t b_tmp = 0;
    for (size_t i = 0; i < 5; ++i) {
        b_tmp |= (this->bitset1[i + 3] << i);
    }
    return b_tmp;
}

SrvTest& SrvTest::set_c(const uint32_t c) {
    this->c = c;
    return *this;
}

uint32_t SrvTest::get_c() const {
    return c;
}

SrvTest& SrvTest::set_d(const uint32_t d) {
    for (size_t i = 0; i < 1; ++i) {
        this->bitset2[i + 0] = d & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_d() const {
    uint32_t d_tmp = 0;
    for (size_t i = 0; i < 1; ++i) {
        d_tmp |= (this->bitset2[i + 0] << i);
    }
    return d_tmp;
}

SrvTest& SrvTest::set_e(const uint32_t e) {
    for (size_t i = 0; i < 14; ++i) {
        this->bitset2[i + 1] = e & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_e() const {
    uint32_t e_tmp = 0;
    for (size_t i = 0; i < 14; ++i) {
        e_tmp |= (this->bitset2[i + 1] << i);
    }
    return e_tmp;
}

SrvTest& SrvTest::set_f(const uint32_t f) {
    for (size_t i = 0; i < 1; ++i) {
        this->bitset2[i + 15] = f & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_f() const {
    uint32_t f_tmp = 0;
    for (size_t i = 0; i < 1; ++i) {
        f_tmp |= (this->bitset2[i + 15] << i);
    }
    return f_tmp;
}

SrvTest& SrvTest::set_g(const uint32_t g) {
    for (size_t i = 0; i < 31; ++i) {
        this->bitset2[i + 16] = g & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_g() const {
    uint32_t g_tmp = 0;
    for (size_t i = 0; i < 31; ++i) {
        g_tmp |= (this->bitset2[i + 16] << i);
    }
    return g_tmp;
}

SrvTest& SrvTest::set_h(const uint32_t h) {
    for (size_t i = 0; i < 1; ++i) {
        this->bitset2[i + 47] = h & (1 << i);
    }
    return *this;
}

uint32_t SrvTest::get_h() const {
    uint32_t h_tmp = 0;
    for (size_t i = 0; i < 1; ++i) {
        h_tmp |= (this->bitset2[i + 47] << i);
    }
    return h_tmp;
}

SrvTest& SrvTest::set_pote(const SrvTest::Pote pote) {
    this->pote = pote;
    return *this;
}

SrvTest::Pote SrvTest::get_pote() const {
    return pote;
}

SrvTest& SrvTest::set_pote2(const SrvTest::Pote2 pote2) {
    this->pote2 = pote2;
    return *this;
}

SrvTest::Pote2 SrvTest::get_pote2() const {
    return pote2;
}

SrvTest SrvTest::create(const uint32_t& a, const uint32_t& b, const uint32_t& c, const uint32_t& d, const uint32_t& e, const uint32_t& f, const uint32_t& g, const uint32_t& h, const SrvTest::Pote& pote, const SrvTest::Pote2& pote2) {
    SrvTest packet;
    packet.set_a(a);
    packet.set_b(b);
    packet.set_c(c);
    packet.set_d(d);
    packet.set_e(e);
    packet.set_f(f);
    packet.set_g(g);
    packet.set_h(h);
    packet.set_pote(pote);
    packet.set_pote2(pote2);
    return packet;
}

SrvTest SrvTest::create(const uint8_t* buffer) {
    CRoseReader reader(buffer, CRosePacket::size(buffer));
    return SrvTest(reader);
}

std::unique_ptr<SrvTest> SrvTest::allocate(const uint8_t* buffer) {
    CRoseReader reader(buffer, CRosePacket::size(buffer));
    return std::make_unique<SrvTest>(reader);
}

bool SrvTest::pack(CRoseBasePolicy& writer) const {
    if (!writer.set_bitset(bitset1)) {
        return false;
    }
    if (!writer.set_uint32_t(c)) {
        return false;
    }
    if (!writer.set_bitset(bitset2)) {
        return false;
    }
    if (!writer.set_iserialize(pote)) {
        return false;
    }
    if (!writer.set_iserialize(pote2)) {
        return false;
    }
    return true;
}

constexpr size_t SrvTest::size() {
    size_t size = 0;
    size += 8 / 8; // bitset1
    size += sizeof(uint32_t); // c
    size += 48 / 8; // bitset2
    size += Pote::size(); // pote
    size += Pote2::size(); // pote2
    return size;
}

