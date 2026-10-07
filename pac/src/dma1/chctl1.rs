#[doc = "Register `CHCTL1` reader"]
pub type R = crate::R<Chctl1Spec>;
#[doc = "Register `CHCTL1` writer"]
pub type W = crate::W<Chctl1Spec>;
#[doc = "Field `SINC` reader - desc SINC"]
pub type SincR = crate::FieldReader;
#[doc = "Field `SINC` writer - desc SINC"]
pub type SincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINC` reader - desc DINC"]
pub type DincR = crate::FieldReader;
#[doc = "Field `DINC` writer - desc DINC"]
pub type DincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRPTEN` reader - desc SRPTEN"]
pub type SrptenR = crate::BitReader;
#[doc = "Field `SRPTEN` writer - desc SRPTEN"]
pub type SrptenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRPTEN` reader - desc DRPTEN"]
pub type DrptenR = crate::BitReader;
#[doc = "Field `DRPTEN` writer - desc DRPTEN"]
pub type DrptenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNSEQEN` reader - desc SNSEQEN"]
pub type SnseqenR = crate::BitReader;
#[doc = "Field `SNSEQEN` writer - desc SNSEQEN"]
pub type SnseqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNSEQEN` reader - desc DNSEQEN"]
pub type DnseqenR = crate::BitReader;
#[doc = "Field `DNSEQEN` writer - desc DNSEQEN"]
pub type DnseqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIZE` reader - desc HSIZE"]
pub type HsizeR = crate::FieldReader;
#[doc = "Field `HSIZE` writer - desc HSIZE"]
pub type HsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LLPEN` reader - desc LLPEN"]
pub type LlpenR = crate::BitReader;
#[doc = "Field `LLPEN` writer - desc LLPEN"]
pub type LlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLPRUN` reader - desc LLPRUN"]
pub type LlprunR = crate::BitReader;
#[doc = "Field `LLPRUN` writer - desc LLPRUN"]
pub type LlprunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc SRPTEN"]
    #[inline(always)]
    pub fn srpten(&self) -> SrptenR {
        SrptenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DRPTEN"]
    #[inline(always)]
    pub fn drpten(&self) -> DrptenR {
        DrptenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SNSEQEN"]
    #[inline(always)]
    pub fn snseqen(&self) -> SnseqenR {
        SnseqenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DNSEQEN"]
    #[inline(always)]
    pub fn dnseqen(&self) -> DnseqenR {
        DnseqenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc HSIZE"]
    #[inline(always)]
    pub fn hsize(&self) -> HsizeR {
        HsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc LLPEN"]
    #[inline(always)]
    pub fn llpen(&self) -> LlpenR {
        LlpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc LLPRUN"]
    #[inline(always)]
    pub fn llprun(&self) -> LlprunR {
        LlprunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc SINC"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SincW<'_, Chctl1Spec> {
        SincW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc DINC"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DincW<'_, Chctl1Spec> {
        DincW::new(self, 2)
    }
    #[doc = "Bit 4 - desc SRPTEN"]
    #[inline(always)]
    pub fn srpten(&mut self) -> SrptenW<'_, Chctl1Spec> {
        SrptenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DRPTEN"]
    #[inline(always)]
    pub fn drpten(&mut self) -> DrptenW<'_, Chctl1Spec> {
        DrptenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc SNSEQEN"]
    #[inline(always)]
    pub fn snseqen(&mut self) -> SnseqenW<'_, Chctl1Spec> {
        SnseqenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DNSEQEN"]
    #[inline(always)]
    pub fn dnseqen(&mut self) -> DnseqenW<'_, Chctl1Spec> {
        DnseqenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc HSIZE"]
    #[inline(always)]
    pub fn hsize(&mut self) -> HsizeW<'_, Chctl1Spec> {
        HsizeW::new(self, 8)
    }
    #[doc = "Bit 10 - desc LLPEN"]
    #[inline(always)]
    pub fn llpen(&mut self) -> LlpenW<'_, Chctl1Spec> {
        LlpenW::new(self, 10)
    }
    #[doc = "Bit 11 - desc LLPRUN"]
    #[inline(always)]
    pub fn llprun(&mut self) -> LlprunW<'_, Chctl1Spec> {
        LlprunW::new(self, 11)
    }
    #[doc = "Bit 12 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, Chctl1Spec> {
        IeW::new(self, 12)
    }
}
#[doc = "desc CHCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl1Spec;
impl crate::RegisterSpec for Chctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl1::R`](R) reader structure"]
impl crate::Readable for Chctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`chctl1::W`](W) writer structure"]
impl crate::Writable for Chctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL1 to value 0x1000"]
impl crate::Resettable for Chctl1Spec {
    const RESET_VALUE: u32 = 0x1000;
}
