#[doc = "Register `OCERW` reader"]
pub type R = crate::R<OcerwSpec>;
#[doc = "Register `OCERW` writer"]
pub type W = crate::W<OcerwSpec>;
#[doc = "Field `CHBUFEN` reader - desc CHBUFEN"]
pub type ChbufenR = crate::FieldReader;
#[doc = "Field `CHBUFEN` writer - desc CHBUFEN"]
pub type ChbufenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLBUFEN` reader - desc CLBUFEN"]
pub type ClbufenR = crate::FieldReader;
#[doc = "Field `CLBUFEN` writer - desc CLBUFEN"]
pub type ClbufenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MHBUFEN` reader - desc MHBUFEN"]
pub type MhbufenR = crate::FieldReader;
#[doc = "Field `MHBUFEN` writer - desc MHBUFEN"]
pub type MhbufenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MLBUFEN` reader - desc MLBUFEN"]
pub type MlbufenR = crate::FieldReader;
#[doc = "Field `MLBUFEN` writer - desc MLBUFEN"]
pub type MlbufenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LMCH` reader - desc LMCH"]
pub type LmchR = crate::BitReader;
#[doc = "Field `LMCH` writer - desc LMCH"]
pub type LmchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMCL` reader - desc LMCL"]
pub type LmclR = crate::BitReader;
#[doc = "Field `LMCL` writer - desc LMCL"]
pub type LmclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMMH` reader - desc LMMH"]
pub type LmmhR = crate::BitReader;
#[doc = "Field `LMMH` writer - desc LMMH"]
pub type LmmhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMML` reader - desc LMML"]
pub type LmmlR = crate::BitReader;
#[doc = "Field `LMML` writer - desc LMML"]
pub type LmmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCECH` reader - desc MCECH"]
pub type McechR = crate::BitReader;
#[doc = "Field `MCECH` writer - desc MCECH"]
pub type McechW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCECL` reader - desc MCECL"]
pub type MceclR = crate::BitReader;
#[doc = "Field `MCECL` writer - desc MCECL"]
pub type MceclW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc CHBUFEN"]
    #[inline(always)]
    pub fn chbufen(&self) -> ChbufenR {
        ChbufenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc CLBUFEN"]
    #[inline(always)]
    pub fn clbufen(&self) -> ClbufenR {
        ClbufenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc MHBUFEN"]
    #[inline(always)]
    pub fn mhbufen(&self) -> MhbufenR {
        MhbufenR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc MLBUFEN"]
    #[inline(always)]
    pub fn mlbufen(&self) -> MlbufenR {
        MlbufenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc LMCH"]
    #[inline(always)]
    pub fn lmch(&self) -> LmchR {
        LmchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc LMCL"]
    #[inline(always)]
    pub fn lmcl(&self) -> LmclR {
        LmclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc LMMH"]
    #[inline(always)]
    pub fn lmmh(&self) -> LmmhR {
        LmmhR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc LMML"]
    #[inline(always)]
    pub fn lmml(&self) -> LmmlR {
        LmmlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MCECH"]
    #[inline(always)]
    pub fn mcech(&self) -> McechR {
        McechR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc MCECL"]
    #[inline(always)]
    pub fn mcecl(&self) -> MceclR {
        MceclR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CHBUFEN"]
    #[inline(always)]
    pub fn chbufen(&mut self) -> ChbufenW<'_, OcerwSpec> {
        ChbufenW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc CLBUFEN"]
    #[inline(always)]
    pub fn clbufen(&mut self) -> ClbufenW<'_, OcerwSpec> {
        ClbufenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc MHBUFEN"]
    #[inline(always)]
    pub fn mhbufen(&mut self) -> MhbufenW<'_, OcerwSpec> {
        MhbufenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc MLBUFEN"]
    #[inline(always)]
    pub fn mlbufen(&mut self) -> MlbufenW<'_, OcerwSpec> {
        MlbufenW::new(self, 6)
    }
    #[doc = "Bit 8 - desc LMCH"]
    #[inline(always)]
    pub fn lmch(&mut self) -> LmchW<'_, OcerwSpec> {
        LmchW::new(self, 8)
    }
    #[doc = "Bit 9 - desc LMCL"]
    #[inline(always)]
    pub fn lmcl(&mut self) -> LmclW<'_, OcerwSpec> {
        LmclW::new(self, 9)
    }
    #[doc = "Bit 10 - desc LMMH"]
    #[inline(always)]
    pub fn lmmh(&mut self) -> LmmhW<'_, OcerwSpec> {
        LmmhW::new(self, 10)
    }
    #[doc = "Bit 11 - desc LMML"]
    #[inline(always)]
    pub fn lmml(&mut self) -> LmmlW<'_, OcerwSpec> {
        LmmlW::new(self, 11)
    }
    #[doc = "Bit 12 - desc MCECH"]
    #[inline(always)]
    pub fn mcech(&mut self) -> McechW<'_, OcerwSpec> {
        McechW::new(self, 12)
    }
    #[doc = "Bit 13 - desc MCECL"]
    #[inline(always)]
    pub fn mcecl(&mut self) -> MceclW<'_, OcerwSpec> {
        MceclW::new(self, 13)
    }
}
#[doc = "desc OCERW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcerwSpec;
impl crate::RegisterSpec for OcerwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocerw::R`](R) reader structure"]
impl crate::Readable for OcerwSpec {}
#[doc = "`write(|w| ..)` method takes [`ocerw::W`](W) writer structure"]
impl crate::Writable for OcerwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCERW to value 0"]
impl crate::Resettable for OcerwSpec {}
