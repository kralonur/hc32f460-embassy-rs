#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Field `DSPD` reader - desc DSPD"]
pub type DspdR = crate::FieldReader;
#[doc = "Field `DSPD` writer - desc DSPD"]
pub type DspdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOHSK` reader - desc NZLSOHSK"]
pub type NzlsohskR = crate::BitReader;
#[doc = "Field `NZLSOHSK` writer - desc NZLSOHSK"]
pub type NzlsohskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAD` reader - desc DAD"]
pub type DadR = crate::FieldReader;
#[doc = "Field `DAD` writer - desc DAD"]
pub type DadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PFIVL` reader - desc PFIVL"]
pub type PfivlR = crate::FieldReader;
#[doc = "Field `PFIVL` writer - desc PFIVL"]
pub type PfivlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc DSPD"]
    #[inline(always)]
    pub fn dspd(&self) -> DspdR {
        DspdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NzlsohskR {
        NzlsohskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - desc DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - desc PFIVL"]
    #[inline(always)]
    pub fn pfivl(&self) -> PfivlR {
        PfivlR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DSPD"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DspdW<'_, DcfgSpec> {
        DspdW::new(self, 0)
    }
    #[doc = "Bit 2 - desc NZLSOHSK"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NzlsohskW<'_, DcfgSpec> {
        NzlsohskW::new(self, 2)
    }
    #[doc = "Bits 4:10 - desc DAD"]
    #[inline(always)]
    pub fn dad(&mut self) -> DadW<'_, DcfgSpec> {
        DadW::new(self, 4)
    }
    #[doc = "Bits 11:12 - desc PFIVL"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PfivlW<'_, DcfgSpec> {
        PfivlW::new(self, 11)
    }
}
#[doc = "desc DCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0x0820_0000"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0x0820_0000;
}
